# PyCC User Guide

## Overview
PyCC is a Python interpreter and compiler that combines the ease of use of Python with the performance benefits of compilation through LLVM.

## Installation

### Prerequisites
- LLVM 14 or later
- Rust toolchain

### Building from Source
```bash
git clone <repository-url>
cd pycc
cargo build --release
```

## Language Reference

### Basic Syntax

#### Variables and Assignment
```python
x = 10
name = "PyCC"
is_compiled = True
```

#### Functions
```python
def add(a, b):
    return a + b

result = add(5, 3)
```

#### Control Structures (Planned)
```python
# If statements (not yet implemented)
if x > 0:
    print("Positive")
else:
    print("Non-positive")

# While loops (not yet implemented)
while x > 0:
    x = x - 1
```

### Data Types

#### Integers
```python
x = 42
y = -123
```

#### Floats
```python
pi = 3.14159
negative = -2.5
```

#### Strings
```python
message = "Hello, PyCC!"
```

#### Booleans
```python
is_true = True
is_false = False
```

#### None
```python
empty = None
```

### Operators

#### Arithmetic Operators
- `+` : Addition
- `-` : Subtraction
- `*` : Multiplication
- `/` : Division
- `%` : Modulo
- `**` : Exponentiation

#### Comparison Operators
- `==` : Equal to
- `!=` : Not equal to
- `<` : Less than
- `>` : Greater than
- `<=` : Less than or equal to
- `>=` : Greater than or equal to

#### Logical Operators
- `and` : Logical AND
- `or` : Logical OR
- `not` : Logical NOT

### Built-in Functions
```python
print("Hello, World!")  # Output to console
```

## Compiler Usage

### Direct Execution
```bash
pycc run input.py
```

### Basic Compilation
```bash
pycc compile input.py -o output
```

### Optimization Levels
```bash
pycc compile input.py -O3 -o output  # High optimization
pycc compile input.py -O0 -o output  # No optimization (default)
```

### Output LLVM IR
```bash
pycc compile input.py --emit-llvm -o output.ll  # Output LLVM IR instead of executable
```

## Examples

### Hello World
```python
x = 42
print(x)
```

### Arithmetic Expressions
```python
a = 10
b = 5
c = a + b * 2
print(c)  # Outputs: 20
```

### Simple Function
```python
def add(x, y):
    return x + y

result = add(3, 4)
print(result)  # Outputs: 7
```

## Error Messages

### Lexical Errors
- "Invalid character": Character not recognized by lexer
- "Unterminated string": String missing closing quote

### Syntax Errors
- "Expected ':'": Missing colon in control structures
- "Mismatched parentheses": Unbalanced parentheses

### Semantic Errors
- "Undefined variable": Variable used before declaration
- "Function not found": Function called but not defined

## Performance Tips

### Type Inference
PyCC performs better when types can be inferred early:
```python
# Good - types are clear
x = 10
y = 20
result = x + y

# Less optimal - types are ambiguous
x = get_value()  # Type unknown until runtime
```

### Function Usage
- Use functions to organize code
- Avoid deeply nested function calls for better optimization
- Prefer iterative solutions over deeply recursive ones

## Current Implementation Status

### Implemented Features
- Variable assignments with basic data types (integers, floats, strings, booleans, None)
- Arithmetic expressions with operator precedence
- Function definitions and calls
- Print statements
- Direct execution mode (interpreter)
- LLVM IR generation and compilation to executables
- Optimization levels (0-3)

### Not Yet Implemented
- Control structures (if/else, while loops)
- Lists, dictionaries, and complex data structures
- Module system
- Standard library expansion
- Exception handling
- Object-oriented programming features
- For loops with iterators

### Future Enhancements
- Support for complex data structures
- Module system
- Standard library expansion
- Exception handling
- Object-oriented programming features
- For loops with iterators
- Advanced standard library
- Language server protocol implementation
- Interactive REPL

## Troubleshooting

### Common Issues
1. **Compilation fails**: Check for syntax errors in source code
2. **Runtime errors**: Ensure all variables are properly initialized
3. **Performance issues**: Use optimization flags (-O2, -O3)

### Getting Help
- Check examples in the examples/ directory
- Refer to language documentation
- Report bugs to the issue tracker