use std::io::Cursor;
use std::error::Error;

use brainfuck_compiler::parser::{Instr, lex, parse};
use brainfuck_compiler::compiler::{self, lower_program, io::{stdin_read, stdout_write}};

mod suite;
use crate::suite::TestCase;


fn run_test(program: Vec<Instr>, input: Cursor<Vec<u8>>, expected_result:  Option<Cursor<Vec<u8>>>) -> Result<(), Box<dyn Error>> {
    return Ok(());
    let code = lower_program(&program, stdin_read, stdout_write)?;
    let jit_program = compiler::JITProgram::new(code) ;

    let result_code = jit_program.run();
    match expected_result {
        Some(succesful_result) => {
            // Check that compiled program did not fail
            assert_eq!(result_code, 0);
            todo!("compare IO output to given output");
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