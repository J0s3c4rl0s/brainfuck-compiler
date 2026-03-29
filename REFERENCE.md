# Brainfuck reference 

## Model 
- 30k cell array of integers (usually one byte)
- One pointer initialized to 0

Henceforth "cell value" is the value of the cell at the current pointer.

## Control flow 
- [Control begins at the first command, if any; when a command has control, it is executed, and by default control passes to the following command, if any; if and when no command has control, the program terminates.
](https://brainfuck.org/brainfuck.html)

## Commands
- `+`/`-` increment/decrement current cell-value (Value may (or may not) loop)
-  `>`/`<` move pointer left/right (undefined behaviour if at relevant border)
- `[` Check cell value: 
    - If zero: Skip to next matching `]` (N.B. Beware nesting)
- `]` Check vell value : 
    - If nonzero: Return to previous matching `]`
- `.` Output cell value (print, define for environment)
- `,` Read one byte input, overwrite current cell value