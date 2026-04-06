use num_traits::PrimInt;

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
pub enum Op {
    Inc(u8), // +
    Dec(u8), // -
    Left(usize), // < 
    Right(usize), // >
    Loop(Vec<Op>), // [...]
    Print, // .
    Read, // ,
}

fn count_remaining_tokens<F, N>(
    tokens: &[Token],
    target: Token,
    make_op: F,
) -> Option<(Op, &[Token])>
where
    N : PrimInt,
    F: Fn(N) -> Op,
{
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

    Some((make_op(count), rest))
}

fn parse_one(tokens: &[Token]) -> Option<(Op, &[Token])> {
    match tokens {
        [] => None,

        // Accumulate repeated operations
        [Token::Inc, rest @ ..] => count_remaining_tokens(rest, Token::Inc, Op::Inc),
        [Token::Dec, rest @ ..] => count_remaining_tokens(rest, Token::Dec, Op::Dec),
        [Token::Left, rest @ ..] => count_remaining_tokens(rest, Token::Left, Op::Left),
        [Token::Right, rest @ ..] => count_remaining_tokens(rest, Token::Right, Op::Right),

        // loops
        [Token::LeftBracket, rest @ ..] => {
            let (body, rest) = parse_block(rest);

            match rest {
                [Token::RightBracket, rest @ ..] => {
                    Some((Op::Loop(body), rest))
                }
                _ => panic!("Unmatched '['"),
            }
        }

        // end of current block
        [Token::RightBracket, _rest @ ..] => None,

        // single ops
        [Token::Print, rest @ ..] => Some((Op::Print, rest)),
        [Token::Read, rest @ ..] => Some((Op::Read, rest)),
    }
}

fn parse_block(mut tokens: &[Token]) -> (Vec<Op>, &[Token]) {
    let mut ops = Vec::new();

    while let Some((op, rest)) = parse_one(tokens) {
        ops.push(op);
        tokens = rest;
    }

    (ops, tokens)
}

pub fn parse(tokens: &[Token]) -> Vec<Op> {
    let (ops, _) = parse_block(tokens);
    ops
}