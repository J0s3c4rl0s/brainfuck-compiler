use std::io::{Read, Write};

use crate::utils::Op;


pub struct Interpreter<R: Read, W: Write> {
    // IO
    input : R,
    output: W,
    // Memory
    cells : Vec<u8>,
    cell_pointer : usize,
}

impl<R: Read, W: Write> Interpreter<R, W> {
    pub fn new(input : R, output: W) -> Self {
        Self { input, output, cells: vec![0; 30000], cell_pointer: 0}
    }

    pub fn run(&mut self, program: &[Op]){
        self.exec(program)
    }

    fn exec(&mut self, ops : &[Op]){
        for op in ops {
            // No bounds checking, relying on rust panicking at under/overflows
            match op {
                Op::Inc(n) => self.cells[self.cell_pointer] += n,
                Op::Dec(n) => self.cells[self.cell_pointer] -= n,
                
                Op::Left(n) => self.cell_pointer -= n,
                Op::Right(n) => self.cell_pointer += n,
                
                Op::Loop(ops) => {
                    while self.cells[self.cell_pointer] != 0 {
                        self.exec(ops);
                    }
                },
                
                Op::Print => {
                    self.output
                        .write_all(&[self.cells[self.cell_pointer]])
                        .expect("Failed to write");
                },
                Op::Read => {
                    let mut buf = [0u8; 1];
                    let input_byte = match self.input.read_exact(&mut buf) {
                        Ok(()) => buf[0],
                        _ => 0, // EOF behavior, maybe panic instead?
                    };
                    //println!("Read byte: {input_byte}");
                    let pointer = self.cell_pointer.to_owned();
                    self.cells[pointer] = input_byte;
                },
            }
        }
    }

    pub fn get_output(self) -> W {
        self.output
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use crate::{interpreter::*, utils::*};

    fn test_program(program_str: &str, input_str: &[u8], output_str: &[u8],) {
        let program = parse(&lex(program_str));

        let input = Cursor::new(input_str);
        let output = Cursor::new(Vec::new());

        let mut interpreter = Interpreter::new(input, output);
        interpreter.run(&program);

        assert_eq!(interpreter.get_output().into_inner(), output_str);
    }


    #[test]
    fn test_echo() {
        test_program(
            "+[,.]", 
            b"abc", 
            b"abc\0");
    }

    #[test]
    fn test_lotoken() {
        test_program(
            "++[>++[>.+<-]<-]", 
            b"", 
            b"\0");
    }

    // Courtesy of https://brainfuck.org/tests.b 

    #[test]
    fn test_double_io() {
        test_program(
            ">,>+++++++++,>+++++++++++[<++++++<++++++<+>>>-]<<.>.<<-.>.>.<<.", 
            b"\n\0", 
            b"LB\nLB\n");
    }

    #[test]
    fn test_array_big_enough() {
        test_program(
            "++++[>++++++<-]>[>+++++>+++++++<<-]>>++++<[[>[[>>+<<-]<]>>>-]>-[>+>+<<-]>]
+++++[>+++++++<<++>-]>.<<.", 
            b"", 
            b"#\n");
    }

    #[test]
    fn test_some_bs() {
        test_program(
            "[]++++++++++[>>+>+>++++++[<<+<+++>>>-]<<<<-]
[>>+<<]>[>>]<<<<[>++<[-]]>.>.", 
            b"", 
            b"H\n");
    }
}