use crate::utils::Op;

enum MergeResult {
    Replace(Op),
    RemoveBoth,
    KeepBoth,
}

fn merge_in_program(program : Vec<Op>) -> Vec<Op> {
    let mut result: Vec<Op> = Vec::new();

    for curr_op in program {
        match result.pop() {
            // If previous operation was a loop then recurse into loop sub-program and continue as usual
            Some(Op::Loop(inner_program)) => {
                result.push(Op::Loop(merge_in_program(inner_program)));
                result.push(curr_op);
            }
            Some(prev_op) => {
                match merge_ops(&prev_op, &curr_op) {
                    MergeResult::Replace(op) => {
                        result.push(op);
                    },
                    MergeResult::KeepBoth => {
                        result.push(prev_op);
                        result.push(curr_op);
                    },
                    MergeResult::RemoveBoth => continue,
                }
            }
            None => {
                result.push(curr_op);
            }
        }
    }

    result

}

fn merge_ops(op1 : &Op, op2 : &Op) -> MergeResult{
    match (op1, op2) {
        // Inc/Dec
        (Op::Inc(n), Op::Dec(m)) => merge_inc_dec(n, m),
        (Op::Dec(n), Op::Inc(m)) => merge_inc_dec(m, n),
        (Op::Inc(n), Op::Inc(m)) => MergeResult::Replace(Op::Inc(n + m)),
        (Op::Dec(n), Op::Dec(m)) => MergeResult::Replace(Op::Inc(n + m)),
        
        // Pointer shifts
        (Op::Left(n), Op::Right(m)) => merge_shift(n, m),
        (Op::Right(n), Op::Left(m)) => merge_shift(m, n),
        (Op::Left(n), Op::Left(m)) => MergeResult::Replace(Op::Left(n + m)),
        (Op::Right(n), Op::Right(m)) => MergeResult::Replace(Op::Right(n + m)),

        // Default case, leave operations unchanged
        (_, _) => MergeResult::KeepBoth,
    }
}

fn merge_inc_dec(n: &u8, m: &u8) -> MergeResult {
    let res = (n.clone() as i16) - (m.clone() as i16);
    if res == 0 {
        MergeResult::RemoveBoth
    } else if res < 0 {
        MergeResult::Replace(Op::Dec(res as u8))
    } else {
        MergeResult::Replace(Op::Inc((-res) as u8))
    }
}

fn merge_shift(n: &usize, m: &usize) -> MergeResult {
    // Number should never exceed 30,000 so cast should be safe (no wrapping) 
    let res = (n.clone() as isize) - (m.clone() as isize);
    if res == 0 {
        MergeResult::RemoveBoth
    } else if res < 0 {
        MergeResult::Replace(Op::Dec(res as u8))
    } else {
        MergeResult::Replace(Op::Inc((-res) as u8))
    }
}