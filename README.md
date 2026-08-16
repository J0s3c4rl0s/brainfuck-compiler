# brainfuck-compiler
Brainfuck compiler and interpreter with different optimizations for fun and learning.

## How to use
### Compiling the project
Build using cargo
`cargo build`

Executable found in `target/debug/brainfuck-compiler`. 

### Running a brainfuck file using CLI
Simply run 

`target/debug/brainfuck-compiler run -c PATH_TO_BRAINFUCK_FILE`

`-c` flag toggles whether it uses the compiler or interpreter

### Example programs
1. `test_echo.b` simply prints to stdout the bytes it reads from stdin, press `Ctrl + C` to quit the program

### Running test suite 
To run all tests 

`cargo test`

Running only one test file (e.g. `compiler` integration test file) 

`cargo test --test compiler`
## Project details

### Interpreters
- Token Interpreter : Stupid naive interpreter with no optimizations
- Interpreter : Runs after proper parser, main "optimization" is +++ -> Inc(3) etc. Base interpreter 

### Compiler 
- JIT compiler in `compiler.rs`

### Optimizations 
- Merge operations: synthesize successive inc/dec shiftLeft/shiftRight operations into one (or none if they cancel out)


<!-- ### TODOS:
- Check issues for up to date ideas I have but in general:
- Expand test cases and include more negative test cases
- Develop a more standardized mechanism of compiler/program/interpreter failure to handle negative cases
- Add a CLI
- Add performance metrics + scripting to graph performance differences/run experiments -->
