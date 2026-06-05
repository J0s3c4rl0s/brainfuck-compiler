use std::{ffi::c_void, io::Read};

pub struct StdContext;

pub extern "C" fn stdin_read(_ctx: *mut c_void) -> i32 {
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

pub extern "C" fn stdout_write(_ctx: *mut c_void, byte: u8) -> i32 {
    print!("{}", byte as char);
    0
}
