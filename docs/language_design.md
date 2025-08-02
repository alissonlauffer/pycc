# PyCC Language Design

## Overview
PyCC (Python Compiled Compiler) is a Python interpreter and compiler that maintains Python's ease of use while providing the performance benefits of compilation through LLVM.

## Core Features
1. Python-like syntax for familiarity
2. Dynamic typing with compilation-time optimizations
3. Functions as first-class citizens
4. Basic data types: integers, floats, strings, booleans, None
5. Control structures: if/else, while loops *(planned)*
6. Variable assignments and expressions

## Currently Implemented Syntax Examples

### Variable Assignment
```
x = 10
name = "PyCC"
is_compiled = True
empty = None
```

### Functions
```
def add(a, b):
    return a + b

result = add(5, 3)
```

### Expressions
```
result = (a + b) * c
is_valid = x > 0 and y < 10
```

## Planned Syntax Examples

### Control Structures
```
if x > 0:
    print("Positive")
else:
    print("Non-positive")

while x > 0:
    x = x - 1
```

## Type System
- Dynamic typing similar to Python
- Compile-time optimizations based on inferred types
- Runtime type checking for complex scenarios