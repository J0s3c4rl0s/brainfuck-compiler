use cranelift_codegen::CompiledCode;

use crate::io::{IoContext, RuntimeIo};

pub struct JITProgram<'a, IO : RuntimeIo> {
    code: CompiledCode,
    io: &'a mut IO,
    memory: [u8; 30_000],
}

impl<'a, IO: RuntimeIo> JITProgram<'a, IO> {
    pub fn new(code: CompiledCode, io: &'a mut IO) -> Self {
        Self { code, io, memory: [0; 30_000] }
    }
    
    pub fn exec(mut self) -> usize {
        let mut buffer = memmap2::MmapOptions::new()
            .len(self.code.code_buffer().len())
            .map_anon()
            .unwrap();

        buffer.copy_from_slice(self.code.code_buffer());

        let buffer = buffer.make_exec().unwrap();

        let mut io_ctx = IoContext {
            io: self.io
        };

        unsafe {
            let code_fn: unsafe extern "sysv64" fn(*mut u8, *mut std::ffi::c_void) -> usize =
                std::mem::transmute(buffer.as_ptr());

            code_fn(self.memory.as_mut_ptr(), &mut io_ctx as *mut _ as *mut std::ffi::c_void)
        }
    }
}