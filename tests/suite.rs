use std::io::{Cursor, Read, Write};
use brainfuck_compiler::utils::{Op, lex, parse};

struct TestCase<R: Read, W: Write> {
    name: String,
    // Arbitrary or string for I/O?
    input: R,
    // If test should fail then None

    result: Option<W>,
    program: Vec<Op>    
}

impl<R: Read, W: Write> TestCase<R, W> {
    fn new(name: String, program: Vec<Op>, input: R, result: Option<W>) -> Self {
        Self { name, input, result, program }
    }
}

// TODO: There must be a better solution than this....
fn make_tests() -> Vec<TestCase<Cursor<Vec<u8>>, Cursor<Vec<u8>>>> {
    vec![
        TestCase::new(
            "test_echo".to_string(),
            parse(&lex("+[,.]")), 
            Cursor::new(b"abc".to_vec()), 
            Some(Cursor::new(b"abc\0".to_vec()))),
        TestCase::new(
            "test_lotoken".to_string(),
            parse(&lex("++[>++[>.+<-]<-]")), 
            Cursor::new(b"".to_vec()), 
            Some(Cursor::new(b"\0".to_vec()))),
        // Following examples courtesy of https://brainfuck.org/tests.b 
        TestCase::new(
            "test_double_io".to_string(),
            parse(&lex(">,>+++++++++,>+++++++++++[<++++++<++++++<+>>>-]<<.>.<<-.>.>.<<.")), 
            Cursor::new(b"\n\0".to_vec()), 
            Some(Cursor::new(b"LB\nLB\n".to_vec()))),
        TestCase::new(
            "test_array_big_enough".to_string(),
            parse(&lex("++++[>++++++<-]>[>+++++>+++++++<<-]>>++++<[[>[[>>+<<-]<]>>>-]>-[>+>+<<-]>]
// +++++[>+++++++<<++>-]>.<<.")), 
            Cursor::new(b"".to_vec()), 
            Some(Cursor::new(b"#\n".to_vec()))),
        TestCase::new(
            "test_some_bs".to_string(),
            parse(&lex("[]++++++++++[>>+>+>++++++[<<+<+++>>>-]<<<<-]
// [>>+<<]>[>>]<<<<[>++<[-]]>.>.")), 
            Cursor::new(b"".to_vec()), 
            Some(Cursor::new(b"H\n".to_vec()))),
    ]
}