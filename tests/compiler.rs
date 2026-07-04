use std::io::Cursor;
use std::error::Error;

use brainfuck_compiler::io::IoContext;
use brainfuck_compiler::optimizations::merge_operations::merge_in_program;
use brainfuck_compiler::optimizations::optimize;
use brainfuck_compiler::parser::{Instr, lex, parse};
use brainfuck_compiler::compiler::{self, lower_program};

mod suite;
use crate::suite::{TestCase, TestContext};


fn run_test(program: Vec<Instr>, input: Cursor<Vec<u8>>, expected_result:  Option<Cursor<Vec<u8>>>) -> Result<(), Box<dyn Error>> {
    let code = lower_program(&program)?;

    let jit_program = compiler::JITProgram::new(code) ;

    let mut test_ctx = TestContext {
        input: input.into_inner(),
        input_pos: 0,
        output: vec![],
    };

    let mut io_ctx = IoContext {
        io: Box::new(&mut test_ctx)
    };

    // This is so jank
    let io_ctx_ptr = &mut io_ctx as *mut _  as *mut std::ffi::c_void;

    let result_code = jit_program.run(io_ctx_ptr);

    match expected_result {
        Some(succesful_result) => {
            // Check that compiled program did not fail
            assert_eq!(result_code, 0);
            assert_eq!(test_ctx.output, succesful_result.into_inner());
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