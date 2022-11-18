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
Coming soon

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
