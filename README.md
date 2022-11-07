# Rust AST
## What is it?
Rust AST is a school project and simple implementation of an Abstract Syntax Tree walker in Rust. The langauge is designed to be fun and is not very useful. Abstract Syntax Tree walkers use the visitor design pattern.

## Grammar Rules
When designing the AST, I originally came up with a set of grammar rules:
```
program = statement*

statement = expr | var_dec | print
expr = expr + expr | expr - expr | expr * expr | expr / expr | expr % expr | expr ^ expr | expr == expr | expr != expr | expr > expr | expr < expr | expr >= expr | expr <= expr | expr && expr | expr || expr | !expr | (expr) | var | number | string | function_call | builtin_call
```