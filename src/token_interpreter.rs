use std::io::{Read, Write};

use crate::utils::Token;

pub struct TokenInterpreter<R: Read, W: Write> {
    // IO
    input : R,
    output: W,
    // Memory
    cells : Vec<u8>,
    cell_pointer : usize,
    // Control flow
    program : Vec<Token>,
    instruction_pointer : usize, 
}

impl<R: Read, W: Write> TokenInterpreter<R, W> {
    pub fn new(input: R, output: W, program: Vec<Token>) -> Self {
        Self { input, output, cells: vec![0; 30000], instruction_pointer: 0, cell_pointer: 0, program }
    }
     
    pub fn run(mut self) -> W{
        while self.instruction_pointer < self.program.len() {
            self.handle_token();
            self.instruction_pointer += 1;
        }
        self.output
    }


    fn read_token(&self) -> Token {
        // Panic if there is no instruction
        if self.instruction_pointer >= self.program.len() {
            panic!("Instruction pointer OOB");
        }

        self.program[self.instruction_pointer]
    }

    // Function for executing one command
    fn handle_token(&mut self) {
        let token = self.read_token();
        //println!("Handling tokeneration: {:?}", token);
        match token {
            Token::Inc => self.cells[self.cell_pointer] += 1,
            Token::Dec => self.cells[self.cell_pointer] -= 1,
            Token::Left => self.left(),
            Token::Right => self.right(),
            Token::LeftBracket => self.cond_left_bracket(),
            Token::RightBracket => self.cond_right_bracket(),
            Token::Print => self.print(),
            Token::Read => self.read(),
        }
    }

    fn left(&mut self) {
        if self.cell_pointer == 0 {
            panic!("Memory pointer is already 0");
        }
        self.cell_pointer -= 1;
    }
    
    fn right(&mut self) {
        if self.cell_pointer >= 300000 {
            panic!("Memory pointer is already 30000");
        }
        self.cell_pointer += 1;
    }
    
    fn cond_left_bracket(&mut self) {
        let skip = self.cells[self.cell_pointer] == 0;
        //println!("{skip}");
        if skip {
            self.skip_to_next_right();
        }
    }
    
    fn skip_to_next_right(&mut self){
        self.instruction_pointer += 1;
        match self.read_token() {
            // Move to next right, and then the one after
            Token::LeftBracket => {
                self.skip_to_next_right();
                self.skip_to_next_right();
            }, 
            // Done! 
            Token::RightBracket => return, 
            // Move on to next character
            _ => {
                self.skip_to_next_right();
            },
        }
    }
    
    fn cond_right_bracket(&mut self) {
        let ret = self.cells[self.cell_pointer] != 0;
        
        if ret {
            self.return_to_last_left();
        } 
    }
    
    fn return_to_last_left(&mut self) {
        self.instruction_pointer -= 1;
        match self.read_token() {
            // Move to prev left, and then the one before
            Token::RightBracket  => {
                self.return_to_last_left();
                self.return_to_last_left();
            }, 
            // Done! 
            Token::LeftBracket => return, 
            // Move on to prev character
            _ => {
                self.return_to_last_left();
            },
        }
    }

    // Rely on provided write functionality
    fn print(&mut self) {
        let value = self.cells[self.cell_pointer];
        self.output.write_all(&[value]).expect("Failed to write");
        //println!("Printing: {value}");
    }
    
    // Rely on provided read functionality
    fn read(&mut self) {
        let mut buf = [0u8; 1];
        let input_byte = match self.input.read_exact(&mut buf) {
            Ok(()) => buf[0],
            _ => 0, // EOF behavior, maybe panic instead?
        };
        //println!("Read byte: {input_byte}");
        self.cells[self.cell_pointer] = input_byte;
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use crate::{token_interpreter::*, utils::lex};

    fn test_program(program_str: &str, input_str: &[u8], output_str: &[u8],) {
        let program = lex(program_str);

        let input = Cursor::new(input_str);
        let output = Cursor::new(Vec::new());

        let interpreter = TokenInterpreter::new(input, output, program);

        assert_eq!(interpreter.run().into_inner(), output_str);
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