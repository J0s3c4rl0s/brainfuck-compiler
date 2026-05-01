use std::usize;

use num_traits::PrimInt;

use crate::error::ParseError;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token {
    Inc, // +
    Dec, // -
    Left, // < 
    Right, // >
    LeftBracket, // [
    RightBracket, // ]
    Print, // .
    Read, // ,
}

pub fn lex(program: &str) -> Vec<Token> {
    program.chars().filter_map(|c| {
        match c {
            '+' => Some(Token::Inc),
            '-' => Some(Token::Dec),
            '<' => Some(Token::Left),
            '>' => Some(Token::Right),
            '[' => Some(Token::LeftBracket),
            ']' => Some(Token::RightBracket),
            '.' => Some(Token::Print),
            ',' => Some(Token::Read),
            _ => None, // ignore everything else
        }
    }).collect()
}

#[derive(Clone, Debug)]
pub struct Instr {
    pub op: Op,
    pub pos: usize
}

#[derive(Clone, Debug)]
pub enum Op { 
    Inc(u8), // +
    Dec(u8), // -
    Left(usize), // < 
    Right(usize), // >
    Loop(Vec<Instr>), // [...]
    Print, // .
    Read, // ,
}

type Result<T> = std::result::Result<T, ParseError>;

// Write an actual counter and then maybe a combiner 
fn count_remaining_tokens<N>(tokens: &[Token], target: Token,) -> (N, &[Token])
where
    N : PrimInt {
    let mut count : N = N::one();
    let mut rest = tokens;

    while let [t, next @ ..] = rest {
        if *t == target {
            count = count + N::one();
            rest = next;
        } else {
            break;
        }
    }

    (count, rest)
}

fn parse_repeat_tokes<F, N>(
    tokens: &[Token],
    target: Token,
    make_op: F,
    pos: usize,
) -> Result<Option<(Op, &[Token], usize)>>
where
    N : PrimInt,
    F: Fn(N) -> Op,
{
    let (count, rest) = count_remaining_tokens::<N>(tokens, target);

    let count_cast: usize = count
        .to_usize()
        .ok_or(ParseError::TooManySymbols { pos })?;
    
    Ok(Some((make_op(count), 
        rest, 
        pos + count_cast)))
}

// Should this return an Instr or Op?
fn parse_instr(tokens: &[Token], pos: usize) -> Result<Option<(Op, &[Token], usize)>> {
    match tokens {
        [] => Ok(None),

        // Accumulate repeated operations
        // Reduce code duplication?
        [Token::Inc, rest @ ..] => parse_repeat_tokes(rest, Token::Inc, Op::Inc, pos),
        [Token::Dec, rest @ ..] => parse_repeat_tokes(rest, Token::Dec, Op::Dec, pos),
        [Token::Left, rest @ ..] => parse_repeat_tokes(rest, Token::Left, Op::Left, pos),
        [Token::Right, rest @ ..] => parse_repeat_tokes(rest, Token::Right, Op::Right, pos),

        // loops
        [Token::LeftBracket, rest @ ..] => {
            let (body, rest, pos) = parse_block(rest, pos + 1)?;

            match rest {
                [Token::RightBracket, rest @ ..] => {
                    Ok(Some((Op::Loop(body), rest, pos)))
                }
                _ => Err(ParseError::UnmatchedOpenBracket { pos })
                //panic!("Unmatched '['"),
            } 
        }

        // end of current block
        [Token::RightBracket, _rest @ ..] => Ok(None),

        // single ops
        [Token::Print, rest @ ..] => Ok(Some((Op::Print, rest, pos))),
        [Token::Read, rest @ ..] => Ok(Some((Op::Read, rest, pos))),
    }
}

fn parse_block(mut tokens: &[Token], mut pos: usize) -> Result<(Vec<Instr>, &[Token], usize)> {
    let mut instrs = Vec::new();

    while let Some((op, rest, new_pos)) = parse_instr(tokens, pos)? {
        instrs.push(Instr { op, pos });
        pos = new_pos;
        tokens = rest;
    }

   Ok((instrs, tokens, pos))
}

pub fn parse(tokens: &[Token]) -> Result<Vec<Instr>> {
    let (instrs, _, _) = parse_block(tokens, 0)?;
    Ok(instrs)
}

#[cfg(test)]
mod test {
    #[test]
    fn todo() {
        todo!("Write tests to make sure position info is correct");
    }
}