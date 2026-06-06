# brainfuck-compiler
Set of brainfuck interpreter/compilers for fun and to compare the speedups 

## Interpreters
- Token Interpreter : Stupid naive interpreter with no optimizations
- Interpreter : Runs after proper parser, main "optimization" is +++ -> Inc(3) etc. Base interpreter 

## Compiler 
- JIT compiler in `compiler.rs`

## Optimizations 
- Merge operations: synthesize successive inc/dec shiftLeft/shiftRight operations into one (or none if they cancel out)


## TODOS:
- Check issues for up to date ideas I have but in general:
- Expand test cases and include more negative test cases
- Develop a more standardized mechanism of compiler/program/interpreter failure to handle negative cases
- Add a CLI
- Add performance metrics + scripting to graph performance differences/run experiments
