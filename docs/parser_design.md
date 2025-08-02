# Parser Design for PyCC

## Overview
The parser is responsible for converting a stream of tokens from the lexer into an Abstract Syntax Tree (AST). The AST represents the syntactic structure of the program and is used by subsequent phases of the compiler.

## Current Grammar Implementation

### Program Structure
```
program ::= statement*

statement ::= assignment
           | return_statement
           | expression_statement
           | function_definition

assignment ::= IDENTIFIER '=' expression

function_definition ::= 'def' IDENTIFIER '(' parameters? ')' ':' return_statement
parameters ::= IDENTIFIER (',' IDENTIFIER)*

return_statement ::= 'return' expression?

expression_statement ::= expression

expression ::= additive
additive ::= multiplicative (('+' | '-') multiplicative)*
multiplicative ::= primary (('*' | '/' | '%') primary)*
primary ::= INTEGER
          | FLOAT
          | STRING
          | BOOLEAN
          | NONE
          | IDENTIFIER function_call?
          | '(' expression ')'

function_call ::= '(' arguments? ')'
arguments ::= expression (',' expression)*
```

## AST Node Types

### Program Nodes
- Program: Root node containing statements
- Function: Function definition with name, parameters, and body

### Statement Nodes
- Assignment: Variable assignment (identifier = expression)
- Return: Return statement with optional expression
- ExpressionStatement: Expression as a statement

### Expression Nodes
- Binary: Binary operations (addition, subtraction, etc.)
- Literal: Constants (integers, floats, strings, booleans, none)
- Identifier: Variable references
- Call: Function calls with arguments

## Current Implementation

### Parser Interface
The parser has a simple interface:
- `parse_program()`: Returns the root AST node for the program
- `parse_expression()`: Parse a single expression

### Error Handling
The parser handles syntax errors such as:
- Unexpected tokens
- Mismatched parentheses
- Invalid expressions

### Precedence Parsing
We use precedence climbing for handling operator precedence correctly.

## Dependencies
- Lexer (for token stream)
- AST node definitions