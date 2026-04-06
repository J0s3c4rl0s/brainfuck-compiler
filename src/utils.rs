#[derive(Clone, Copy, Debug)]
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