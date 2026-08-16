use std::ffi::c_void;

// Generic Trait for IO implementations
pub trait RuntimeIo {
    fn read(&mut self) -> Option<u8>;
    fn write(&mut self, byte: u8);
}

// Wrappers to pass to the compiler as pointers
pub struct IoContext<'a> {
    pub io: Box<&'a mut dyn RuntimeIo>,
}

pub extern "C" fn bf_write(ctx: *mut c_void, byte: u8) -> i32 {
    let ctx = unsafe {
        &mut *(ctx as *mut IoContext)
    };

    ctx
        .io
        .write(byte);
    0
}

pub extern "C" fn bf_read(ctx: *mut c_void) -> i32 {
    let ctx = unsafe {
        &mut *(ctx as *mut IoContext)
    };

    match ctx.io.read() {
        Some(b) => {
            b as i32
        },
        // Assume that its EOF if None
        None => 0,
    }
}