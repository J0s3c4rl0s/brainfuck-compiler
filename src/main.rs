use std::{fs, io::{Read, Write}, println, todo};

use brainfuck_compiler::{compiler::{JITProgram, lower_program}, interpreter::Interpreter, io::RuntimeIo, parser::{lex, parse}};
use clap::{Parser, Subcommand};

/// Program to interpret or JIT compile brainfuck files
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    // Perhaps optimization level or a list of optimizations down the line? 

    /// Merge optimization 
    #[arg(short, long)]
    merge: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Compile {
        /// Path to the brainfuck file from current root
        file_path: String,

        /// File to write to 
        #[arg(short, long)]
        output_file: String,
    },

    Run {
        /// Path to the brainfuck file from current root
        file_path: String,

        /// Use compiler, default is interpreter
        #[arg(short, long)]
        compile: bool,
    },

    TestFolder {
        path: String,

        /// Use compiler, default is interpreter
        #[arg(short, long)]
        compile: bool,
    }
}

struct IOStd;

impl RuntimeIo for IOStd {
    fn read(&mut self) -> Option<u8> {
        let mut buf: [u8; 1] = [0];
        match std::io::stdin().read_exact(&mut buf) {
            Ok(_) => Some(buf[0]),
            Err(err) => match err.kind() {
                std::io::ErrorKind::UnexpectedEof => Some(0),
                _ => None,
            },
        }
    }

    fn write(&mut self, byte: u8) {
        std::io::stdout().write(&[byte]).unwrap();
    }
}



// Example: `bf run -i`
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Compile { file_path: _, output_file: _ } => todo!("Implement compiling to an external file, probably figure out linker"),
        // Commands::Compile { file_path, output_file, merge } => todo!(),
        
        Commands::Run { file_path, compile } => {
            // println!("Reading from file ")
            let source_program = fs::read_to_string(&file_path)?;
            let parsed = parse(&lex(&source_program))?;
            
            // Feels a bit silly but alas
            let mut io = IOStd{};
            if compile {
                JITProgram::new(
                    lower_program(&parsed)?, 
                    &mut io)
                    .exec();
                return Ok(());

            } else {
                Interpreter::new(&mut io)
                    .exec(&parsed)?
            }
        },
        
        /*
        IDEA:
            Give folder 
            Read in folder all bf files
            Some format for the input and the expected output
            Loop over each of these triplets and run them
            Receive options for optimizations  
        */
        Commands::TestFolder { path , compile: _compile } => {
            let files = read_all_files(path);
            for (filename, _input, _expected_output, _code) in files {
                println!("Running testcase: {filename}");
                
                // setup_runner(compile)
                //     .test(
                //         &parse(&lex(&code))?, 
                //         input, 
                //         Some(expected_output))?;   
            }
        },
    }

    Ok(())
}



fn read_all_files(_path: String) -> Vec<(String, Vec<u8>, Vec<u8>, String)> {
    todo!("include some logic for defining a negative test case, ie when to give a none instead of an expected output");
}
