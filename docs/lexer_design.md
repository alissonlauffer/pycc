# Lexer Design for PyCC

## Overview
The lexer (lexical analyzer) is responsible for converting the source code into a stream of tokens. These tokens are then consumed by the parser to build the Abstract Syntax Tree (AST).

## Implemented Token Types

### Literals
- INTEGER: `123`, `0`, `-456`
- FLOAT: `3.14`, `-2.5`
- STRING: `"hello"`, `'world'`
- BOOLEAN: `true`, `false`
- NONE: `None`

### Identifiers
- IDENTIFIER: `variable_name`, `functionName`

### Keywords
- `def`, `if`, `else`, `while`, `return`, `true`, `false`, `None`, `and`, `or`, `not`

### Operators
- Arithmetic: `+`, `-`, `*`, `/`, `%`, `**`
- Comparison: `==`, `!=`, `<`, `>`, `<=`, `>=`
- Assignment: `=`
- Logical: `and`, `or`, `not`

### Delimiters
- `(`, `)`, `{`, `}`, `[`, `]`
- `,`, `:`, `;`

### Whitespace and Comments
- Whitespace is skipped
- Comments start with `#` and continue to end of line

## Current Implementation

### Token Structure
Each token contains:
- Type (enum)
- Literal value (for literals)

### Lexer Interface
The lexer implements a simple interface with:
- `next_token()` - Returns the next token in the stream
- `peek_char()` - Returns the next character without consuming it

### Error Handling
The lexer handles lexical errors such as:
- Invalid characters
- Unterminated strings
- Invalid number formats

## Dependencies
- None (core part of compiler)