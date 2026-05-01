use crate::parser::{Instr, Op};

enum MergeResult {
    Replace(Instr),
    RemoveBoth,
    KeepBoth,
}

pub fn merge_in_program(program : Vec<Instr>) -> Vec<Instr> {
    let mut result: Vec<Instr> = Vec::new();

    for curr_instr in program {
        match result.pop() {
            // If previous operation was a loop then recurse into loop sub-program and continue as usual
            Some(Instr { op: Op::Loop(inner_program), pos: inner_pos }) => {
                result.push(Instr { op: Op::Loop(merge_in_program(inner_program)), pos: inner_pos });
                result.push(curr_instr);
            }
            Some(prev_instr) => {
                match merge_ops(&prev_instr, &curr_instr) {
                    MergeResult::Replace(instr) => {
                        result.push(instr);
                    },
                    MergeResult::KeepBoth => {
                        result.push(prev_instr);
                        result.push(curr_instr);
                    },
                    MergeResult::RemoveBoth => continue,
                }
            }
            None => {
                result.push(curr_instr);
            }
        }
    }

    result

}

fn merge_ops(op1 : &Instr, op2 : &Instr) -> MergeResult{
    // Write 
    match (op1, op2) {
        // Inc/Dec
        (Instr { op: Op::Inc(n), pos }, Instr { op: Op::Dec(m), pos: _ }) => merge_inc_dec(*n, *m, *pos),
        (Instr { op: Op::Dec(n), pos }, Instr { op: Op::Inc(m), pos: _ }) => merge_inc_dec(*m, *n, *pos),
        (Instr { op: Op::Inc(n), pos }, Instr { op: Op::Inc(m), pos: _ }) => MergeResult::Replace(Instr { op: Op::Inc(n + m), pos: *pos }),
        (Instr { op: Op::Dec(n), pos }, Instr { op: Op::Dec(m), pos: _ }) => MergeResult::Replace(Instr { op: Op::Inc(n + m), pos: *pos }),
        
        // Pointer shifts
        (Instr { op: Op::Left(n), pos }, Instr { op: Op::Right(m), pos : _ }) => merge_shift(*n, *m, *pos),
        (Instr { op: Op::Right(n), pos }, Instr { op: Op::Left(m), pos: _ }) => merge_shift(*m, *n, *pos),
        (Instr { op: Op::Left(n), pos }, Instr { op: Op::Left(m), pos: _ }) => MergeResult::Replace(Instr { op: Op::Left(n + m), pos: *pos }),
        (Instr { op: Op::Right(n), pos }, Instr { op: Op::Right(m), pos: _ }) => MergeResult::Replace(Instr { op: Op::Right(n + m), pos: *pos }),

        // Default case, leave operations unchanged
        (_, _) => MergeResult::KeepBoth,
    }
}
    
fn merge_inc_dec(n: u8, m: u8, pos: usize) -> MergeResult {
    let res = (n as i16) - (m as i16);
    if res == 0 {
        MergeResult::RemoveBoth
    } else if res < 0 {
        MergeResult::Replace(Instr { op: Op::Dec(res as u8), pos })
    } else {
        MergeResult::Replace(Instr { op: Op::Inc((-res) as u8), pos })
    }
}

fn merge_shift(n: usize, m: usize, pos: usize) -> MergeResult {
    // Number should never exceed 30,000 so cast should be safe (no wrapping) 
    let res = (n.clone() as isize) - (m.clone() as isize);
    if res == 0 {
        MergeResult::RemoveBoth
    } else if res < 0 {
        MergeResult::Replace(Instr { op: Op::Left(res as usize), pos })
    } else {
        MergeResult::Replace(Instr { op: Op::Right((-res) as usize), pos })
    }
}