use std::{fs, io::{Cursor, stdin, stdout}, os::raw::c_void};

use brainfuck_compiler::{compiler::{self, JITProgram, io::{stdin_read, stdout_write}}, interpreter, parser::{self, parse}};
use clap::Parser;

/// Program to interpret or JIT compile brainfuck files
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the brainfuck file from current root
    file_path: String,

    /// Use compiler, default is interpreter
    #[arg(short, long)]
    compile: bool,
}

// Example: `bf run -i`
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // println!("Reading from file ")
    let source_program = fs::read_to_string(&args.file_path)?;
    let program = parser::parse(&parser::lex(&source_program))?;
    
    if args.compile {
        println!("Compiling and running program {}", &args.file_path);

        let compiled_code = compiler::lower_program(&program, stdin_read, stdout_write)?;
        let jit_exec = compiler::JITProgram::new(compiled_code);

        // SAFETY: io_context argument is not used for stdin or stdout
        jit_exec.run(0 as *mut c_void);
    }
    else {
        println!("Interpreting program {}!", &args.file_path);
        let mut interpreter = interpreter::Interpreter::new(stdin(), stdout());

        interpreter.run(&program)?;
    }

    Ok(())
}
