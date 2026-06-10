pub mod merge_operations;

use crate::parser::Instr;

pub fn optimize(program : Vec<Instr>, optimizations : Vec<fn(Vec<Instr>) -> Vec<Instr>>) -> Vec<Instr>{
    let mut program = program;
    for opt in optimizations {
        program = opt(program);
    }
    program
}