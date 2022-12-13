# Linear Algebra AST
## What is it?
Linear Algebra AST is a simple AST interpreter that handles parsing of simple expressions.

## Features
Current and planned(tbd) features include:
[X] Read files and output result of statements
[X] Read individual statements from REPL
[X] Declare global variables as scalars and matrices
[X] Basic Scalar Operations (add, sub, multiply, divide)
[X] Basic Matrix Operations (add, sub, multiply, scalar multiply)
[X] Groupings
[X] Print

## Grammar Rules
```
program = statement*;

statement = (expr | let | print | newline);
expr = literal | unary | binary | grouping | identifier
```

## How to Use
Run
```
cargo build
```
in the root linear-algebra-ast directory to build the project for your target system.

After building with cargo, you can run the tool on the command line if the exe file is in your PATH variable or the current directory.

You can then run the REPL by simply entering
```
linear-algebra-ast
```
or you can run a file with
```
linear-algebra-ast file.la
```

## Examples
```
let a = 2;
let B = [1 0|0 1];
print a * B;
```
Results in:
```
| 2 0 |
| 0 2 |
```

```
let A = [1 2|3 4];
print ref(A);
```
Results in:
```
| -2 1 |
| 1.5 -0.5 |
```

## LICENSE
MIT License, feel free to use or contribute what you wish.
