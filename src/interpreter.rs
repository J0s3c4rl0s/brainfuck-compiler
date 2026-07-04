
use crate::io::RuntimeIo;
use crate::parser::{Instr, Op};
use crate::error::{InterpreterError, RuntimeError};

type Result<T> = std::result::Result<T, InterpreterError>;

pub struct Interpreter<'a, IO> where IO : RuntimeIo {
    // IO
    io_ctx: &'a mut IO,
    // Memory
    cells : Vec<u8>,
    cell_pointer : usize,
}

impl<'a, IO> Interpreter<'a,IO> where IO : RuntimeIo {
    pub fn new(io_ctx: &'a mut IO) -> Self {
        Self { io_ctx, cells: vec![0; 30000], cell_pointer: 0}
    }

    pub fn exec(&mut self, ops : &[Instr]) -> Result<()>{
        for Instr { op, pos } in ops {
            // Lets me use shorthand initialization
            let pos = *pos;
            
            // Have bounds checking but it might still panick at large enough n
            match op {
                // Add 
                Op::Inc(n) => {
                    let curr_cell_value = self.cells[self.cell_pointer];
                    if curr_cell_value >= u8::MAX - n {
                        return Err(InterpreterError::Runtime(RuntimeError::IntegerOverflow { pos, left: curr_cell_value, right: *n }));
                    } 
        
                    self.cells[self.cell_pointer] += n
                },
                Op::Dec(n) => {
                    let curr_cell_value = self.cells[self.cell_pointer];
                    if curr_cell_value < *n {
                        return Err(InterpreterError::Runtime(RuntimeError::IntegerUnderflow { pos, left: curr_cell_value, right: *n }));
                    }

                    self.cells[self.cell_pointer] -= n
                },
                
                // Shift
                Op::Left(n) => {
                    if self.cell_pointer < *n {
                        return Err(InterpreterError::Runtime(RuntimeError::PointerOutOfBounds { pos, index: self.cell_pointer as isize - *n as isize }));
                    }
                    self.cell_pointer -= n
                },
                Op::Right(n) => {
                    if self.cell_pointer >= usize::MAX - *n {
                        // Shitty cast, might silently underflow 
                        return Err(InterpreterError::Runtime(RuntimeError::PointerOutOfBounds { pos, index: (self.cell_pointer + *n) as isize }));
                    }
                    self.cell_pointer += n
                },

                // Loop
                Op::Loop(ops) => {
                    while self.cells[self.cell_pointer] != 0 {
                        self.exec(ops)?;
                    }
                },
                
                // IO operations
                Op::Print => {
                    // Add a result to write for IO errors?
                    self.io_ctx.write(self.cells[self.cell_pointer])
                    
                    // match self.output.write_all(&[self.cells[self.cell_pointer]]) {
                    //     Ok(()) => (),
                    //     Err(err) => return Err(InterpreterError::Runtime(RuntimeError::IoError { pos: pos, error: err }))
                    // }
                },
                Op::Read => {
                    // let mut buf = [0u8; 1];
                    // let input_byte = match self.input.read_exact(&mut buf) {
                    //     Ok(()) => buf[0],
                    //     Err(err) => match err.kind() {
                    //         // EOF behaviour
                    //         std::io::ErrorKind::UnexpectedEof => 0,
                    //         _ => return Err(InterpreterError::Runtime(RuntimeError::IoError { pos: pos, error: err })),
                    //     }, // EOF behavior, maybe panic instead?
                    // };
                    
                    let input_byte = match self.io_ctx.read() {
                        Some(b) => b,
                        // Always assume None is EOF?
                        None => 0,
                    } ;
                    //println!("Read byte: {input_byte}");
                    let pointer = self.cell_pointer.to_owned();
                    self.cells[pointer] = input_byte;
                },
            }
        }

        Ok(())
    }
}
