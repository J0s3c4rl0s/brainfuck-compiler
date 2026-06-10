use std::io::{Read, Write};

use crate::{error::{InterpreterError, ParseError, RuntimeError}, parser::Token};

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
     
    pub fn run(mut self) -> Result<W, InterpreterError>{
        while self.instruction_pointer < self.program.len() {
            self.handle_token()?;
            self.instruction_pointer += 1;
        }
        Ok(self.output)
    }


    fn read_token(&self) -> Result<Token, InterpreterError> {
        // Panic if there is no instruction
        if self.instruction_pointer >= self.program.len() {
            panic!("Instruction pointer OOB");
        }

        Ok(self.program[self.instruction_pointer])
    }

    // Function for executing one command
    fn handle_token(&mut self) -> Result<(), InterpreterError> {
        let token = self.read_token()?;
        match token {
            // Either inline other function calls or make helper methods for these as well
            Token::Inc => {
                let curr_cell_value = self.cells[self.cell_pointer];
                if curr_cell_value == u8::MAX {
                    return Err(InterpreterError::Runtime(RuntimeError::IntegerOverflow { pos: self.instruction_pointer, left: curr_cell_value, right: 1 }));
                }

                self.cells[self.cell_pointer] = curr_cell_value + 1;
                Ok(())
            },
            Token::Dec => {
                let curr_cell_value = self.cells[self.cell_pointer];
                if curr_cell_value == u8::MIN {
                    return Err(InterpreterError::Runtime(RuntimeError::IntegerUnderflow { pos: self.instruction_pointer, left: curr_cell_value, right: 1 }));
                }
                
                self.cells[self.cell_pointer] = curr_cell_value - 1;
                Ok(())
            },
            Token::Left => self.left(),
            Token::Right => self.right(),
            Token::LeftBracket => self.cond_left_bracket(),
            Token::RightBracket => self.cond_right_bracket(),
            Token::Print => self.print(),
            Token::Read => self.read(),
        }
    }

    fn left(&mut self) -> Result<(), InterpreterError> {
        if self.cell_pointer == 0 {
            return Err(InterpreterError::Runtime(RuntimeError::PointerOutOfBounds { pos: self.instruction_pointer, index: -1 }))
        }
        self.cell_pointer -= 1;
        Ok(())
    }
    
    fn right(&mut self) -> Result<(), InterpreterError> {
        if self.cell_pointer >= 300000 {
            return Err(InterpreterError::Runtime(RuntimeError::PointerOutOfBounds { pos: self.instruction_pointer, index:  300001}));
        }
        self.cell_pointer += 1;
        Ok(())
    }
    
    fn cond_left_bracket(&mut self) -> Result<(), InterpreterError> {
        let skip = self.cells[self.cell_pointer] == 0;
        if skip {
            self.skip_to_next_right(self.instruction_pointer)
        } else {
            Ok(())
        }
    }
    
    // Rewrite to use a stack for better clarity?
    fn skip_to_next_right(&mut self, left_bracket_pos: usize) -> Result<(), InterpreterError> {
        self.instruction_pointer += 1;
        match self.read_token() {
                Ok(token) => match token {
                // Move to next right, and then the one after
                Token::LeftBracket => {
                    self.skip_to_next_right(self.instruction_pointer)?;
                    self.skip_to_next_right(left_bracket_pos)
                }, 
                // Done! 
                Token::RightBracket => return Ok(()), 
                // Move on to next character
                _ => {
                    self.skip_to_next_right(left_bracket_pos)
                },
            },
            // If we have run out of characters we have an unmatched bracket
            Err(_err) => Err(InterpreterError::Parser(ParseError::UnmatchedOpenBracket { pos: left_bracket_pos })),
        }
    }
    
    fn cond_right_bracket(&mut  self) -> Result<(), InterpreterError>  {
        let ret = self.cells[self.cell_pointer] != 0;
        
        if ret {
            self.return_to_last_left(self.instruction_pointer)
        } else {
            Ok(())
        }
    }
    
    // Rewrite to use a stack for better clarity?
    fn return_to_last_left(&mut  self, right_bracket_pos: usize) -> Result<(), InterpreterError>  {
        self.instruction_pointer -= 1;
        match self.read_token() {
            Ok(token) => match token {
                // Move to prev left, and then the one before
                Token::RightBracket  => {
                    self.return_to_last_left(self.instruction_pointer)?;
                    self.return_to_last_left(right_bracket_pos)
                }, 
                // Done! 
                Token::LeftBracket => return Ok(()), 
                // Move on to prev character
                _ => {
                    self.return_to_last_left(right_bracket_pos)
                },
            },
            Err(_) => Err(InterpreterError::Parser(ParseError::UnmatchedCloseBracket { pos: right_bracket_pos })),
        }
    }

    // Rely on provided write functionality
    fn print(&mut  self) -> Result<(), InterpreterError>  {
        let value = self.cells[self.cell_pointer];
        match self.output.write_all(&[value]) {
            Ok(()) => Ok(()),
            Err(io_error) => Err(InterpreterError::Runtime(RuntimeError::IoError { pos: self.instruction_pointer, error: io_error })),
        }
    }
    
    // Rely on provided read functionality
    fn read(&mut  self) -> Result<(), InterpreterError>  {
        // todo!("Add errors for reading input");
        let mut buf = [0u8; 1];
        let input_byte = match self.input.read_exact(&mut buf) {
            Ok(()) => buf[0],
            Err(err) => {
                match err.kind() {
                    // If you reach EOF then just read 0
                    std::io::ErrorKind::UnexpectedEof => 0,
                    _ => return Err(InterpreterError::Runtime(RuntimeError::IoError { pos: self.instruction_pointer, error: err })),
                }
            },
        };
        self.cells[self.cell_pointer] = input_byte;
        Ok(())
    }
}
