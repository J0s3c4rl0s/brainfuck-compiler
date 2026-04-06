use std::io::{Read, Write};

use crate::utils::Op;


pub struct Interpreter<R: Read, W: Write> {
    // IO
    input : R,
    output: W,
    // Memory
    cells : Vec<u8>,
    cell_pointer : usize,
    // Control flow
    program : Vec<Op>,
    // instruction_pointer : usize, 
}

impl<R: Read, W: Write> Interpreter<R, W> {
    pub fn new(input : R, output: W, program: Vec<Op>) -> Self {
        Self { input, output, cells: vec![0; 30000], cell_pointer: 0, program}
    }

    pub fn run(&mut self){
        // Expensive copy here? 
        self.exec(&self.program.to_vec())
    }

    fn exec(&mut self, ops : &[Op]){
        for op in ops {
            // No bounds checking, relying on rust panicking at under/overflows
            match op {
                Op::Inc(n) => self.cells[self.cell_pointer] += n,
                Op::Dec(n) => self.cells[self.cell_pointer] -= n,
                Op::Left(n) => self.cell_pointer -= n,
                Op::Right(n) => self.cell_pointer += n,
                Op::Loop(ops) => todo!(),
                Op::Print => todo!(),
                Op::Read => todo!(),
            }
        }
    }

    
}