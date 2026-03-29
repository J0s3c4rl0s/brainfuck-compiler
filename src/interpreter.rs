use std::io::{Read, Write};


#[derive(Clone, Copy, Debug)]
pub enum Op {
    Inc, // +
    Dec, // -
    Left, // < 
    Right, // >
    LeftBracket, // [
    RightBracket, // ]
    Print, // .
    Read, // ,
}

pub fn lex(program: &str) -> Vec<Op> {
    program.chars().filter_map(|c| {
        match c {
            '+' => Some(Op::Inc),
            '-' => Some(Op::Dec),
            '<' => Some(Op::Left),
            '>' => Some(Op::Right),
            '[' => Some(Op::LeftBracket),
            ']' => Some(Op::RightBracket),
            '.' => Some(Op::Print),
            ',' => Some(Op::Read),
            _ => None, // ignore everything else
        }
    }).collect()
}

// Internal memory model for the interpreter
// Want to compare interpreter to different compilers so lets be mildly efficient
pub struct Interpreter<R: Read, W: Write> {
    input : R,
    output: W,
    cells : Vec<u8>,
    instruction_pointer : usize, 
    cell_pointer : usize,
    program : Vec<Op>,
}

impl<R: Read, W: Write> Interpreter<R, W> {
    pub fn new(input: R, output: W, program: Vec<Op>) -> Self {
        Self { input, output, cells: vec![0; 30000], instruction_pointer: 0, cell_pointer: 0, program }
    }
     
    pub fn run(mut self) -> W{
        while self.instruction_pointer < self.program.len() {
            self.handle_op();
            self.instruction_pointer += 1;
        }
        self.output
    }


    fn read_op(&self) -> Op {
        // Panic if there is no instruction
        if self.instruction_pointer >= self.program.len() {
            panic!("Instruction pointer OOB");
        }

        self.program[self.instruction_pointer]
    }

    // Function for executing one command
    fn handle_op(&mut self) {
        let op = self.read_op();
        //println!("Handling operation: {:?}", op);
        match op {
            Op::Inc => self.increment_cell(),
            Op::Dec => self.decrement_cell(),
            Op::Left => self.left(),
            Op::Right => self.right(),
            Op::LeftBracket => self.cond_left_bracket(),
            Op::RightBracket => self.cond_right_bracket(),
            Op::Print => self.print(),
            Op::Read => self.read(),
        }
    }

    fn increment_cell(&mut self) {
        self.cells[self.cell_pointer] += 1;
    }

    fn decrement_cell(&mut self) {
        self.cells[self.cell_pointer] -= 1;
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
        else {
            // self.instruction_pointer += 1;
        }
    }
    
    fn skip_to_next_right(&mut self){
        self.instruction_pointer += 1;
        match self.read_op() {
            // Move to next right, and then the one after
            Op::LeftBracket => {
                self.skip_to_next_right();
                self.skip_to_next_right();
            }, 
            // Done! 
            Op::RightBracket => return, 
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
        else {
            // self.instruction_pointer += 1;
        }
    }
    
    fn return_to_last_left(&mut self) {
        self.instruction_pointer -= 1;
        match self.read_op() {
            // Move to prev left, and then the one before
            Op::RightBracket  => {
                self.return_to_last_left();
                self.return_to_last_left();
            }, 
            // Done! 
            Op::LeftBracket => return, 
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
    use crate::interpreter::*;

    fn test_program(program_str: &str, input_str: &[u8], output_str: &[u8],) {
        let program = lex(program_str);

        // Input: "abc"
        let input = Cursor::new(input_str);
        let output = Cursor::new(Vec::new());

        let interpreter = Interpreter::new(input, output, program);

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
    fn test_loop() {
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

    // #[should_panic]
    // #[test]
    // fn test_unmatched() {
    // }
}