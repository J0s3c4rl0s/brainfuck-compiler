mod suite;
use std::io::Cursor;

use brainfuck_compiler::interpreter::Interpreter;
use brainfuck_compiler::optimizations::{optimize};
use brainfuck_compiler::optimizations::merge_operations::merge_in_program;
use brainfuck_compiler::parser::{Instr, lex, parse};
use brainfuck_compiler::error::InterpreterError;

use crate::suite::TestCase;

fn run_test(program: Vec<Instr>, input: Cursor<Vec<u8>>, result:  Option<Cursor<Vec<u8>>>) -> Result<(), InterpreterError> {

    let output = Cursor::new(Vec::new());
    let mut interpreter = Interpreter::new(input, output);

    match result {
        Some(succesful_result) => {
            interpreter.run(&program)?;
            assert_eq!(interpreter.get_output().into_inner(), succesful_result.into_inner());
            Ok(())
        }
        None => {
            // If none then we expect some Error 
            assert!(matches!(interpreter.run(&program), Err(_)));
            Ok(())
        }
    }
}

// fn run_optimizations(program_str: &str, input_str: &[u8], output_str: &[u8], optimizations : Vec<fn(Vec<Instr>) -> Vec<Instr>>,) -> Result<(), InterpreterError> {
//     let output = Cursor::new(Vec::new());

//     let mut interpreter = Interpreter::new(input, output);
//     interpreter.run(&program);

//     assert_eq!(interpreter.get_output().into_inner(), output_str);
//     Ok(())
// }

#[test]
fn basic_tests() -> Result<(), InterpreterError> {
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
fn merge_tests() -> Result<(), InterpreterError> {
    let tests = suite::make_tests();

    for test in tests{
        let TestCase {name, input, result, program: program_str} = test;
        println!("Running integration test {name}");
        let program = optimize(parse(&lex(&program_str))?,  vec![merge_in_program]);

        run_test(program, input, result)?;
    }
    Ok(())
}

