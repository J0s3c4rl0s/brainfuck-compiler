use crate::{interpreter, utils::Op};

pub fn optimize(program : Vec<Op>, optimizations : Vec<fn(Vec<Op>) -> Vec<Op>>) -> Vec<Op>{
    let mut program = program;
    for opt in optimizations {
        program = opt(program);
    }
    program
}

enum CombineResult {
    Replace(Op),
    RemoveBoth,
    KeepBoth,
}

fn operation_combining(program : Vec<Op>) -> Vec<Op> {
    let mut result: Vec<Op> = Vec::new();

    for curr_op in program {
        match result.pop() {
            // If previous operation was a loop then recurse into loop sub-program and continue as usual
            Some(Op::Loop(inner_program)) => {
                result.push(Op::Loop(operation_combining(inner_program)));
                result.push(curr_op);
            }
            Some(prev_op) => {
                match combine_ops(&prev_op, &curr_op) {
                    CombineResult::Replace(op) => {
                        result.push(op);
                    },
                    CombineResult::KeepBoth => {
                        result.push(prev_op);
                        result.push(curr_op);
                    },
                    CombineResult::RemoveBoth => continue,
                }
            }
            None => {
                result.push(curr_op);
            }
        }
    }

    result

}

fn combine_ops(op1 : &Op, op2 : &Op) -> CombineResult{
    match (op1, op2) {
        // Inc/Dec
        (Op::Inc(n), Op::Dec(m)) => combine_inc_dec(n, m),
        (Op::Dec(n), Op::Inc(m)) => combine_inc_dec(m, n),
        (Op::Inc(n), Op::Inc(m)) => CombineResult::Replace(Op::Inc(n + m)),
        (Op::Dec(n), Op::Dec(m)) => CombineResult::Replace(Op::Inc(n + m)),
        
        // Pointer shifts
        (Op::Left(n), Op::Right(m)) => combine_shift(n, m),
        (Op::Right(n), Op::Left(m)) => combine_shift(m, n),
        (Op::Left(n), Op::Left(m)) => CombineResult::Replace(Op::Left(n + m)),
        (Op::Right(n), Op::Right(m)) => CombineResult::Replace(Op::Right(n + m)),

        // Default case, leave operations unchanged
        (_, _) => CombineResult::KeepBoth,
    }
}

fn combine_inc_dec(n: &u8, m: &u8) -> CombineResult {
    let res = (n.clone() as i16) - (m.clone() as i16);
    if res == 0 {
        CombineResult::RemoveBoth
    } else if res < 0 {
        CombineResult::Replace(Op::Dec(res as u8))
    } else {
        CombineResult::Replace(Op::Inc((-res) as u8))
    }
}

fn combine_shift(n: &usize, m: &usize) -> CombineResult {
    // Number should never exceed 30,000 so cast should be safe (no wrapping) 
    let res = (n.clone() as isize) - (m.clone() as isize);
    if res == 0 {
        CombineResult::RemoveBoth
    } else if res < 0 {
        CombineResult::Replace(Op::Dec(res as u8))
    } else {
        CombineResult::Replace(Op::Inc((-res) as u8))
    }
}


#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use crate::{interpreter::*, optimization::optimize, utils::*};

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