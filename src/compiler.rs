use cranelift_codegen::CompiledCode;
use cranelift_codegen::ir::{AbiParam, Block, Function, InstBuilder, MemFlags, SigRef, Signature, TrapCode, Type, UserFuncName, Value, condcodes::IntCC, types::I8};
use cranelift_codegen::isa::{self, CallConv};
use cranelift_codegen::settings::{self, Configurable};
use cranelift_codegen::Context;
use cranelift_codegen::control::ControlPlane;
use cranelift_codegen::verify_function;
use cranelift_codegen::CompileError;

use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext, Variable};

use std::sync::Arc;
use std::io::{Read, Write};

use target_lexicon::Triple;

use crate::parser::{Instr, Op};

mod jit;

fn set_flags() -> settings::Flags {
    let mut builder = settings::builder();
    builder.set("opt_level", "speed").unwrap();
    settings::Flags::new(builder)
}

fn setup_function(pointer_type: Type) -> (Function, FunctionBuilderContext) {
    // receive memory address as parameter, and return pointer to io::Error
    let mut sig = Signature::new(CallConv::SystemV);
    sig.params.push(AbiParam::new(pointer_type));
    sig.returns.push(AbiParam::new(pointer_type));
    
    (Function::with_name_signature(UserFuncName::user(0, 0), sig), FunctionBuilderContext::new())
}


fn setup_main_block(function_builder: &mut FunctionBuilder) -> Block {
    let block = function_builder.create_block();
    function_builder.seal_block(block);

    function_builder.append_block_params_for_function_params(block);
    function_builder.switch_to_block(block);
    block
}

extern "C" fn bf_write(byte: u8) -> i32 {
    print!("{}", byte as char);
    0
}

fn define_indirect_print(pointer_type: Type, function_builder: &mut FunctionBuilder) -> (SigRef, Value) {
    let mut write_sig = Signature::new(CallConv::SystemV);
    write_sig.params.push(AbiParam::new(I8));
    write_sig.returns.push(AbiParam::new(pointer_type));
    let write_sig = function_builder.import_signature(write_sig);

    let write_address = bf_write as *const () as i64;
    let write_address = function_builder.ins().iconst(pointer_type, write_address);
    (write_sig, write_address)
}

extern "C" fn bf_read() -> i32 {
    let mut stdin = std::io::stdin();
    let mut buf: [u8; 1] = [0; 1];

    // Are these error codes in line with the interpreter values?
    match stdin.read_exact(&mut buf) {
        Ok(()) =>buf[0] as i32, // byte read
        // IO error or EOF
        Err(err) => match err.kind() {
            std::io::ErrorKind::UnexpectedEof => 0, // EOF behaviour
            _ => -1, // IO Error
        },         
    }
}

fn define_indirect_read(pointer_type: Type, function_builder: &mut FunctionBuilder<'_>) -> (SigRef, Value) {
    let mut read_sig = Signature::new(CallConv::SystemV);
    read_sig.params.push(AbiParam::new(pointer_type));
    read_sig.returns.push(AbiParam::new(pointer_type));
    let read_sig = function_builder.import_signature(read_sig);

    let read_address = bf_read as *const () as i64;
    let read_address = function_builder.ins().iconst(pointer_type, read_address);
    (read_sig, read_address)
}

struct Lowerer<'a> {
    function_builder: FunctionBuilder<'a>,
    pointer: Variable,
    pointer_type: Type,
    memory_address: Value,
    mem_flags: MemFlags,
    write_address: Value,
    write_sig: SigRef,
    read_address: Value,
    read_sig: SigRef,
}

impl<'a> Lowerer<'a> {
    fn lower_add(&mut self, n : i64) {
        let pointer_value = self.function_builder.use_var(self.pointer);
        let cell_address = self.function_builder.ins().iadd(self.memory_address, pointer_value);
        let cell_value = self.function_builder.ins().load(I8, self.mem_flags, cell_address, 0);
        let cell_value = self.function_builder.ins().iadd_imm(cell_value, n);
        self.function_builder.ins().store(self.mem_flags, cell_value, cell_address, 0);

        // Should I add runtime checks for under/overflows? 
    }

    fn lower_shift_left(&mut self, offset : i64) {
        let pointer_value = self.function_builder.use_var(self.pointer);
        let pointer_minus = self.function_builder.ins().iadd_imm(pointer_value, -offset);

        // If pointer_minus < 0 then trap with heap OOB error
        let is_underflow = self.function_builder.ins().icmp_imm(IntCC::SignedLessThanOrEqual, pointer_minus, 0);
        self.function_builder.ins().trapnz(is_underflow, TrapCode::HEAP_OUT_OF_BOUNDS);

        self.function_builder.def_var(self.pointer, pointer_minus);
    }

    fn lower_shift_right(&mut self, offset : i64) {
        let pointer_value = self.function_builder.use_var(self.pointer);
        let pointer_plus = self.function_builder.ins().iadd_imm(pointer_value, offset);

        // If pointer_plus > 30k then trap with heap OOB error
        let is_overflow = self.function_builder.ins().icmp_imm(IntCC::SignedGreaterThanOrEqual, pointer_plus, 30_000);
        self.function_builder.ins().trapnz(is_overflow, TrapCode::HEAP_OUT_OF_BOUNDS);

        self.function_builder.def_var(self.pointer, pointer_plus);
    }

    fn branch_on_loop_cond (&mut self, inner_block: Block, after_block: Block) {
        let pointer_value = self.function_builder.use_var(self.pointer);
        let cell_address = self.function_builder.ins().iadd(self.memory_address, pointer_value);
        let cell_value = self.function_builder.ins().load(I8, self.mem_flags, cell_address, 0);
        // if nonzero then keep looping else quit loop
        // Should these blocks have arguments?
        self.function_builder.ins().brif(cell_value, inner_block, &[], after_block, &[]);
    }

    fn lower_loop(&mut self, inner : &[Instr]) {
        let inner_block = self.function_builder.create_block();
        let after_block = self.function_builder.create_block();

        self.branch_on_loop_cond(inner_block, after_block);
        // Compile loop interior recursively
        self.function_builder.switch_to_block(inner_block);
        self.lower_instrs(inner);
        self.branch_on_loop_cond(inner_block, after_block);
        // Seal both blocks since all predecessors have been defined
        self.function_builder.seal_block(inner_block);
        self.function_builder.seal_block(after_block);

        // compile remaining instructions in after block 
        self.function_builder.switch_to_block(after_block);
    }
    
    fn lower_write(&mut self) {
        let pointer_value = self.function_builder.use_var(self.pointer);
        let cell_address = self.function_builder.ins().iadd(self.memory_address, pointer_value);
        let cell_value = self.function_builder.ins().load(I8, self.mem_flags, cell_address, 0);

        let inst = self.function_builder
            .ins()
            .call_indirect(self.write_sig, self.write_address, &[cell_value]);
        let result = self.function_builder.inst_results(inst)[0];

        // Make this work with enums instead
        // Rewrite this to use Result instead of panicking?
        let trap_code = TrapCode::user(1).unwrap();
        self.function_builder.ins().trapnz(result, trap_code);
    }
    
    fn lower_read(&mut self) {
        let pointer_value = self.function_builder.use_var(self.pointer);
        let cell_address = self.function_builder.ins().iadd(self.memory_address, pointer_value);

        let inst = self.function_builder
            .ins()
            .call_indirect(self.read_sig, self.read_address, &[cell_address]);
        let result = self.function_builder.inst_results(inst)[0];
    }

    fn lower_instrs(&mut self, program: &[Instr]) {
        for b in program.iter() {
            let Instr { op, pos } = b;
            match op {
                // Is this a safe cast? Can I make it safer?
                // Kind of assuming that the pointer type is gonna be 64 bits methinks
                Op::Inc(n) => self.lower_add(*n as i64),
                Op::Dec(n) => self.lower_add(-(*n as i64)),

                // Add compile time bounds checks? (ie if n is > 30k it would always fail)
                // Again is this a safe cast?
                Op::Left(n) => self.lower_shift_left(*n as i64),
                Op::Right(n) => self.lower_shift_right(*n as i64),
                
                Op::Loop(inner) => self.lower_loop(inner),
                
                Op::Print => self.lower_write(),
                Op::Read => self.lower_read(),
            }
        }
    }
    
    fn close_function(mut self) {
        let zero = self.function_builder.ins().iconst(self.pointer_type, 0);
        // Return from function with no error (0)
        self.function_builder.ins().return_(&[zero]);

        self.function_builder.finalize();
    }

    fn new(func : &'a mut Function, func_ctx: &'a mut FunctionBuilderContext, pointer_type: Type) -> Self {
        let mut function_builder = FunctionBuilder::new(func, func_ctx);

        // create the variable `pointer` (it is a offset from memory address)
        let pointer = function_builder.declare_var(pointer_type);

        let block = setup_main_block(&mut function_builder);

        let memory_address = function_builder.block_params(block)[0];

        // initialize pointer to 0 
        let zero = function_builder.ins().iconst(pointer_type, 0);
        function_builder.def_var(pointer, zero);

        let mem_flags = MemFlags::new();

        let (write_sig, write_address) = define_indirect_print(pointer_type, &mut function_builder);

        let (read_sig, read_address) = define_indirect_read(pointer_type, &mut function_builder);

        Self { function_builder, pointer, pointer_type, memory_address, mem_flags, write_address, write_sig, read_address, read_sig }
    }
}

pub fn lower_program(program : &[Instr]) -> Result<CompiledCode, Box<dyn std::error::Error>> {
    let flags = set_flags();
    
    let isa = match isa::lookup(Triple::host()) {
        // Should I really panic here?
        Err(_) => panic!("x86_64 ISA is not avaliable"),
        Ok(isa_builder) => isa_builder.finish(flags).unwrap(),
    };
    let pointer_type = isa.pointer_type();

    let (mut func, mut func_ctx) = setup_function(pointer_type); 

    let mut compiler = Lowerer::new(&mut func, &mut func_ctx, pointer_type);

    compiler.lower_instrs(program);
    compiler.close_function();
    verify_function(&func, &*isa)?;


    let mut ctx = Context::for_function(func);
    let code = match ctx.compile(&*isa, &mut ControlPlane::default()) {
        Ok(x) => x,
        Err(CompileError { inner, func: _ }) => {return Err(Box::new(inner));}
    };

    Ok(code.to_owned())
}

// Based on https://rodrigodd.github.io/2022/11/26/bf_compiler-part3.html