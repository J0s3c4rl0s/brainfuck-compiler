use std::io::{Cursor, Read, Write};

use brainfuck_compiler::io::RuntimeIo;

pub struct TestCase<R: Read, W: Write> {
    pub name: String,
    // Arbitrary or string for I/O?
    pub input: R,
    // If test should fail then None

    pub result: Option<W>,
    pub program: String    
}

impl<R: Read, W: Write> TestCase<R, W> {
    fn new(name: String, program: String, input: R, result: Option<W>) -> Self {
        Self { name, input, result, program }
    }
}


pub struct TestContext {
    pub input: Vec<u8>,
    pub input_pos: usize,
    pub output: Vec<u8>,
}

impl RuntimeIo for TestContext {
    fn read(&mut self) -> Option<u8> {
        let val = self
            .input
            .get(self.input_pos)
            .copied();
        self.input_pos += 1;
        val
    }

    fn write(&mut self, byte: u8) {
        self.output
            .push(byte);
    }
}

// TODO: There must be a better solution than this....
pub fn make_tests() -> Vec<TestCase<Cursor<Vec<u8>>, Cursor<Vec<u8>>>> {
    vec![
        TestCase::new(
            "test_echo".to_string(),
            "+[,.]".to_string(), 
            Cursor::new(b"abc".to_vec()), 
            Some(Cursor::new(b"abc\0".to_vec()))),
        TestCase::new(
            "test_1+1_print".to_string(), 
            "++.".to_string(),
            Cursor::new(b"".to_vec()), 
            Some(Cursor::new(vec![2 as u8]))),  
        TestCase::new(
            "test_1+1-1_print".to_string(), 
            "++-.".to_string(),
            Cursor::new(b"".to_vec()), 
            Some(Cursor::new(vec![1 as u8]))),  
        TestCase::new(
            "test_1+1-1_print".to_string(), 
            "++-.".to_string(),
            Cursor::new(b"".to_vec()), 
            Some(Cursor::new(vec![1 as u8]))),  
        TestCase::new(
            "test_shiftr".to_string(), 
            ">".to_string(),
            Cursor::new(b"".to_vec()), 
            Some(Cursor::new(vec![]))),  
        TestCase::new(
            "test_shiftrl".to_string(), 
            "><".to_string(),
            Cursor::new(b"".to_vec()), 
            Some(Cursor::new(vec![]))),        
        // Following examples courtesy of https://brainfuck.org/tests.b 
        TestCase::new(
            "test_double_io".to_string(),
            ">,>+++++++++,>+++++++++++[<++++++<++++++<+>>>-]<<.>.<<-.>.>.<<.".to_string(), 
            Cursor::new(b"\n\0".to_vec()), 
            Some(Cursor::new(b"LB\nLB\n".to_vec()))),
        TestCase::new(
            "test_array_big_enough".to_string(),
            "++++[>++++++<-]>[>+++++>+++++++<<-]>>++++<[[>[[>>+<<-]<]>>>-]>-[>+>+<<-]>]
// +++++[>+++++++<<++>-]>.<<.".to_string(), 
            Cursor::new(b"".to_vec()), 
            Some(Cursor::new(b"#\n".to_vec()))),
        TestCase::new(
            "test_some_bs".to_string(),
            "[]++++++++++[>>+>+>++++++[<<+<+++>>>-]<<<<-]
// [>>+<<]>[>>]<<<<[>++<[-]]>.>.".to_string(), 
            Cursor::new(b"".to_vec()), 
            Some(Cursor::new(b"H\n".to_vec()))),        
        TestCase::new(
            "test_lotoken".to_string(),
            "++[>++[>.+<-]<-]".to_string(), 
            Cursor::new(b"".to_vec()), 
            Some(Cursor::new(b"\0".to_vec()))),
    ]
}