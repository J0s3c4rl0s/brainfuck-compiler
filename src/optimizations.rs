mod merge_operations;

use crate::{interpreter, utils::Op};

pub fn optimize(program : Vec<Op>, optimizations : Vec<fn(Vec<Op>) -> Vec<Op>>) -> Vec<Op>{
    let mut program = program;
    for opt in optimizations {
        program = opt(program);
    }
    program
}


#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use crate::{interpreter::*, optimizations::optimize, utils::*};

    fn test_optimizations(program_str: &str, input_str: &[u8], output_str: &[u8], optimizations : Vec<fn(Vec<Op>) -> Vec<Op>>,) {
        let program = optimize(parse(&lex(program_str)),  optimizations);

        let input = Cursor::new(input_str);
        let output = Cursor::new(Vec::new());

        let mut interpreter = Interpreter::new(input, output);
        interpreter.run(&program);

        assert_eq!(interpreter.get_output().into_inner(), output_str);
    }

    // Rewrite this so theres a default test suite and I can just hot swap optimizations?


//     #[test]
//     fn test_echo() {
//         test_program(
//             "+[,.]", 
//             b"abc", 
//             b"abc\0");
//     }

//     #[test]
//     fn test_lotoken() {
//         test_program(
//             "++[>++[>.+<-]<-]", 
//             b"", 
//             b"\0");
//     }

//     // Courtesy of https://brainfuck.org/tests.b 

//     #[test]
//     fn test_double_io() {
//         test_program(
//             ">,>+++++++++,>+++++++++++[<++++++<++++++<+>>>-]<<.>.<<-.>.>.<<.", 
//             b"\n\0", 
//             b"LB\nLB\n");
//     }

//     #[test]
//     fn test_array_big_enough() {
//         test_program(
//             "++++[>++++++<-]>[>+++++>+++++++<<-]>>++++<[[>[[>>+<<-]<]>>>-]>-[>+>+<<-]>]
// +++++[>+++++++<<++>-]>.<<.", 
//             b"", 
//             b"#\n");
//     }

//     #[test]
//     fn test_some_bs() {
//         test_program(
//             "[]++++++++++[>>+>+>++++++[<<+<+++>>>-]<<<<-]
// [>>+<<]>[>>]<<<<[>++<[-]]>.>.", 
//             b"", 
//             b"H\n");
//     }
}