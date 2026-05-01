use std::error::Error;

#[derive(Debug)]
// Gotta think of a better name than this
pub enum InterpreterError {
    Parser(ParseError),
    Runtime(RuntimeError)
}

impl From<ParseError> for InterpreterError {
    fn from(value: ParseError) -> Self {
        InterpreterError::Parser(value)
    }
}

impl From<RuntimeError> for InterpreterError {
    fn from(value: RuntimeError) -> Self {
        InterpreterError::Runtime(value)
    }
}

impl std::fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InterpreterError::Parser(parse_error) => parse_error.fmt(f),
            InterpreterError::Runtime(runtime_error) => runtime_error.fmt(f),
        }
    }
}

impl Error for InterpreterError {}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedChar { ch: char, pos: usize },
    UnmatchedOpenBracket { pos: usize },
    UnmatchedCloseBracket { pos: usize },
    // Maybe instead make the error about failed cast (only current use of it)
    TooManySymbols {pos: usize},
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedChar { ch, pos } => write!(f, "Unexpected character {ch} at position {pos}"),
            ParseError::UnmatchedOpenBracket { pos } => write!(f, "Unmatched open bracket at position {pos}"),
            // Do I even need to variants for this? 
            ParseError::UnmatchedCloseBracket { pos } => write!(f, "Unmatched close bracket at position {pos}"),
            ParseError::TooManySymbols { pos } => write!(f, "Too many repeated symbols as {pos}. Usize counter overflowed (failed cast)"),
        }
    }
}
impl Error for ParseError {}

#[derive(Debug)]
pub enum RuntimeError {
    PointerOutOfBounds { pos: usize, index: isize },
    IntegerOverflow { pos: usize, left:  u8, right: u8 },
    IntegerUnderflow { pos: usize, left:  u8, right: u8 },
    // Add info for this error?
    IoError { pos: usize, error: std::io::Error},
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::PointerOutOfBounds { pos, index } => write!(f, "Cell pointer {index} out of bounds (0,30000) at position {pos}"),
            RuntimeError::IntegerOverflow { pos, left, right } => write!(f, "Integer overflow when adding {left} to {right} at position {pos}"),
            RuntimeError::IntegerUnderflow { pos, left, right } => write!(f, "Integer unerflow when subtracting {right} from {left} at position {pos}"),
            RuntimeError::IoError { pos, error } => write!(f, "IO Error at position {pos}\n {error}"),
        }
    }
}
