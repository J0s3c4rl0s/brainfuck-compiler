mod suite;
use std::io::Cursor;

use brainfuck_compiler::utils::lex;
use brainfuck_compiler::token_interpreter::TokenInterpreter;
use brainfuck_compiler::error::InterpreterError;

use crate::suite::TestCase;

fn run_test(test : TestCase<Cursor<Vec<u8>>, Cursor<Vec<u8>>>) -> Result<(), InterpreterError> {
    let TestCase {name, input, result, program: program_str} = test;
    println!("Running integration test {name}");
    let program = lex(&program_str);

    let output = Cursor::new(Vec::new());

    let interpreter = TokenInterpreter::new(input, output, program);

    match result {
        Some(succesful_result) => {
            let result = interpreter.run()?;
            assert_eq!(result.into_inner(), succesful_result.into_inner());
            Ok(())
        }
        None => {
            // If none then we expect some Error 
            assert!(matches!(interpreter.run(), Err(_)));
            Ok(())
        }
    }
}

#[test]
fn basic_tests() -> Result<(), InterpreterError> {
    let tests = suite::make_tests();

    for test in tests{
        run_test(test)?;
    }
    Ok(())
}