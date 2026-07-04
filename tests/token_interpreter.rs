mod suite;
use std::io::Cursor;

use brainfuck_compiler::parser::lex;
use brainfuck_compiler::token_interpreter::TokenInterpreter;
use brainfuck_compiler::error::InterpreterError;

use crate::suite::TestCase;

fn run_test(test : TestCase) -> Result<(), InterpreterError> {
    let TestCase {name, ctx, expected_result, program: program_str} = test;
    println!("Running integration test {name}");
    let program = lex(&program_str);


    let output = Cursor::new(Vec::new());
    // Kinda circuitous way to define this but I dont really care about the token interpreter since it doesnt even operate on the IR
    let interpreter = TokenInterpreter::new(Cursor::new(ctx.input), output, program);

    match expected_result {
        Some(succesful_result) => {
            let result = interpreter.run()?;
            assert_eq!(result.into_inner(), succesful_result);
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