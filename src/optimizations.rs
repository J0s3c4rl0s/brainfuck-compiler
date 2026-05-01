pub mod merge_operations;

use crate::parser::{Instr, Op};

pub fn optimize(program : Vec<Instr>, optimizations : Vec<fn(Vec<Instr>) -> Vec<Instr>>) -> Vec<Instr>{
    let mut program = program;
    for opt in optimizations {
        program = opt(program);
    }
    program
}


#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use crate::{error::InterpreterError, interpreter::*, optimizations::optimize, parser::*};

    
    fn test_optimizations(program_str: &str, input_str: &[u8], output_str: &[u8], optimizations : Vec<fn(Vec<Instr>) -> Vec<Instr>>,) -> Result<(), InterpreterError> {
        let program = optimize(parse(&lex(program_str))?,  optimizations);

        let input = Cursor::new(input_str);
        let output = Cursor::new(Vec::new());

        let mut interpreter = Interpreter::new(input, output);
        interpreter.run(&program);

        assert_eq!(interpreter.get_output().into_inner(), output_str);
        Ok(())
    }
}