use std::io::Cursor;
use std::error::Error;

use brainfuck_compiler::optimizations::merge_operations::merge_in_program;
use brainfuck_compiler::optimizations::optimize;
use brainfuck_compiler::parser::{Instr, lex, parse};
use brainfuck_compiler::compiler::{self, lower_program};

mod suite;
use crate::suite::TestCase;

struct TestContext {
    input: Vec<u8>,
    input_pos: usize,
    output: Vec<u8>,
}

extern "C" fn test_read(
    io_ctx: *mut std::ffi::c_void,
) -> i32 {
    let ctx = unsafe {
        &mut *(io_ctx as *mut TestContext)
    };

    // Are these error codes in line with the interpreter values?
    // todo!("Add logic for handling EOF to align with interpreter semantics");
    if let Some(&byte) = ctx.input.get(ctx.input_pos) {
        ctx.input_pos += 1;
        byte as i32
    } else {
        0
    }
}

extern "C" fn test_write(
    ctx: *mut std::ffi::c_void,
    byte: u8,
) -> i32 {
    let ctx = unsafe {
        &mut *(ctx as *mut TestContext)
    };

    ctx.output.push(byte);
    0
}

fn run_test(program: Vec<Instr>, input: Cursor<Vec<u8>>, expected_result:  Option<Cursor<Vec<u8>>>) -> Result<(), Box<dyn Error>> {
    let code = lower_program(&program, test_read, test_write)?;
    // print assembly
    // println!("{}", code.vcode.clone().unwrap());

    let jit_program = compiler::JITProgram::new(code) ;

    let mut io_ctx = TestContext {
        input: input.into_inner(),
        input_pos: 0,
        output: vec![],
    };

    // This is so jank
    let io_ctx_ptr = &mut io_ctx as *mut _  as *mut std::ffi::c_void;

    let result_code = jit_program.run(io_ctx_ptr);

    match expected_result {
        Some(succesful_result) => {
            // Check that compiled program did not fail
            assert_eq!(result_code, 0);
            assert_eq!(io_ctx.output, succesful_result.into_inner());
            Ok(())
        }
        None => {
            assert_eq!(result_code, 1);
            todo!("Have specific information for what fails? Wrap JIT execution in result?");
            Ok(())
        }
    }
}

#[test]
fn basic_tests() -> Result<(), Box<dyn Error>> {
    let tests = suite::make_tests();

    for test in tests{
        let TestCase {name, input, result, program: program_str} = test;
        println!("Running integration test {name}");
        let program = parse(&lex(&program_str))?;

        run_test(program, input, result)?;
    }
    Ok(())
}

#[test]
fn merge_tests() -> Result<(), Box<dyn Error>> {
    let tests = suite::make_tests();

    for test in tests{
        let TestCase {name, input, result, program: program_str} = test;
        println!("Running integration test {name}");
        let program = optimize(parse(&lex(&program_str))?, vec![merge_in_program]);

        run_test(program, input, result)?;
    }
    Ok(())
}