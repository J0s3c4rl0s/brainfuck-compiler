use std::io::Read;

use cranelift_codegen::{ir::{AbiParam, InstBuilder, SigRef, Signature, Type, Value, types::I8}, isa::CallConv};
use cranelift_frontend::FunctionBuilder;

pub extern "C" fn bf_read() -> i32 {
    let mut stdin = std::io::stdin();
    let mut buf: [u8; 1] = [0; 1];

    // Are these error codes in line with the interpreter values?
    match stdin.read_exact(&mut buf) {
        Ok(()) =>buf[0] as i32, // byte read
        // IO error or EOF
        Err(err) => match err.kind() {
            std::io::ErrorKind::UnexpectedEof => 0, // EOF behaviour
            _ => -1, // IO Error
        },         
    }
}

pub extern "C" fn bf_write(byte: u8) -> i32 {
    print!("{}", byte as char);
    0
}