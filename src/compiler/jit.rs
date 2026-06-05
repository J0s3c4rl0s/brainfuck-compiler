use cranelift_codegen::CompiledCode;

pub struct JITProgram {
    code : CompiledCode,
    memory: [u8; 30_000],
}

impl JITProgram {
    pub fn new(code: CompiledCode) -> Self {
        Self { code, memory: [0; 30_000] }
    }
    
    pub fn run(mut self, io_context : *mut std::ffi::c_void) -> usize {
        let mut buffer = memmap2::MmapOptions::new()
            .len(self.code.code_buffer().len())
            .map_anon()
            .unwrap();

        buffer.copy_from_slice(self.code.code_buffer());

        let buffer = buffer.make_exec().unwrap();

        unsafe {
            let code_fn: unsafe extern "sysv64" fn(*mut u8, *mut std::ffi::c_void) -> usize =
                std::mem::transmute(buffer.as_ptr());

            code_fn(self.memory.as_mut_ptr(), io_context)
        }
    }
}

// #[cfg(test)]
// mod test {
//     use std::error::Error;

//     use crate::{compiler::{self, io::{stdin_read, stdout_write}}, parser::{lex, parse}};
//     use crate::compiler::jit::JITProgram;

//     #[test]
//     fn test_basic() -> Result<(), Box<dyn Error>>{
//         let program = parse(&lex(",+."))?;
        
//         let code = compiler::lower_program(&program, stdin_read, stdout_write)?;

//         let jit_program = JITProgram {code, memory: [0;30_000]};
//         let _result = jit_program.run();
//         Ok(())
//     }
// }
