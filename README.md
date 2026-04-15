# brainfuck-compiler
Set of brainfuck interpreter/compilers for fun and to compare the speedups 

## Interpreters
### Interpreters
- Token Interpreter : Stupid naive interpreter with no optimizations
- Interpreter : Runs after proper parser, main "optimization" is +++ -> Inc(3) etc. Base interpreter 

### Optimizations 
- WIP Combine operations: synthesize successive inc/dec shiftLeft/shiftRight operations into one (or none if they cancel out)






