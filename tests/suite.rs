use brainfuck_compiler::io::RuntimeIo;

pub struct TestCase {
    pub name: String,
    pub ctx: TestContext,
    // If test should fail then None
    pub expected_result: Option<Vec<u8>>,
    pub program: String    
}

impl TestCase {
    fn new(name: String, program: String, ctx: TestContext, expected_result: Option<Vec<u8>>) -> Self {
        Self { name, ctx, expected_result, program }
    }
}

pub struct TestContext {
    pub input: Vec<u8>,
    pub input_pos: usize,
    pub output: Vec<u8>,
}

impl TestContext {
    pub fn new(input: Vec<u8>) -> Self {
        Self { input, input_pos: 0, output: vec![] }
    }
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
pub fn make_tests() -> Vec<TestCase> {
    vec![
        TestCase::new(
            "test_echo".to_string(),
            "+[,.]".to_string(), 
            TestContext::new(b"abc".to_vec()), 
            Some(b"abc\0".to_vec())),
        TestCase::new(
            "test_1+1_print".to_string(), 
            "++.".to_string(),
            TestContext::new(b"".to_vec()), 
            Some(vec![2 as u8])),  
        TestCase::new(
            "test_1+1-1_print".to_string(), 
            "++-.".to_string(),
            TestContext::new(b"".to_vec()), 
            Some(vec![1 as u8])),  
        TestCase::new(
            "test_1+1-1_print".to_string(), 
            "++-.".to_string(),
            TestContext::new(b"".to_vec()), 
            Some(vec![1 as u8])),  
        TestCase::new(
            "test_shiftr".to_string(), 
            ">".to_string(),
            TestContext::new(b"".to_vec()), 
            Some(vec![])),  
        TestCase::new(
            "test_shiftrl".to_string(), 
            "><".to_string(),
            TestContext::new(b"".to_vec()), 
            Some(vec![])),        
        // Following examples courtesy of https://brainfuck.org/tests.b 
        TestCase::new(
            "test_double_io".to_string(),
            ">,>+++++++++,>+++++++++++[<++++++<++++++<+>>>-]<<.>.<<-.>.>.<<.".to_string(), 
            TestContext::new(b"\n\0".to_vec()), 
            Some(b"LB\nLB\n".to_vec())),
        TestCase::new(
            "test_array_big_enough".to_string(),
            "++++[>++++++<-]>[>+++++>+++++++<<-]>>++++<[[>[[>>+<<-]<]>>>-]>-[>+>+<<-]>]
// +++++[>+++++++<<++>-]>.<<.".to_string(), 
            TestContext::new(b"".to_vec()), 
            Some(b"#\n".to_vec())),
        TestCase::new(
            "test_some_bs".to_string(),
            "[]++++++++++[>>+>+>++++++[<<+<+++>>>-]<<<<-]
// [>>+<<]>[>>]<<<<[>++<[-]]>.>.".to_string(), 
            TestContext::new(b"".to_vec()), 
            (Some(b"H\n".to_vec()))),        
        TestCase::new(
            "test_lotoken".to_string(),
            "++[>++[>.+<-]<-]".to_string(), 
            TestContext::new(b"".to_vec()), 
            (Some(b"\0".to_vec()))),
    ]
}