use inkwell::context::Context;
use pycc::ast::*;
use pycc::codegen::CodeGenerator;
use pycc::interpreter::Interpreter;
use pycc::lexer::{Lexer, Token};
use pycc::parser::Parser;
use std::fs;
use tempfile::NamedTempFile;

#[test]
fn test_full_pipeline_simple_program() {
    let source = "x = 42; print(x);";

    // Test lexer
    let mut lexer = Lexer::new(source);
    let tokens: Vec<_> = std::iter::from_fn(|| {
        let token = lexer.next_token();
        if token != Token::Eof {
            Some(token)
        } else {
            None
        }
    })
    .collect();

    assert!(!tokens.is_empty());

    // Test parser
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    match &program {
        Node::Program(prog) => {
            assert!(!prog.statements.is_empty());
        }
        _ => panic!("Expected program node"),
    }

    // Test interpreter
    let mut interpreter = Interpreter::new();
    let _result = interpreter.interpret(&program);
    assert!(_result.is_ok());

    // Test code generation
    let context = Context::create();
    let mut codegen = CodeGenerator::new(&context, "test_module");
    let _result = codegen.compile(&program);
    assert!(_result.is_ok());
}

#[test]
fn test_full_pipeline_arithmetic() {
    let source = "a = 10; b = 5; c = a + b * 2; print(c);";

    // Test parser
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    match &program {
        Node::Program(prog) => {
            assert_eq!(prog.statements.len(), 4); // 3 assignments + 1 print
        }
        _ => panic!("Expected program node"),
    }

    // Test interpreter
    let mut interpreter = Interpreter::new();
    let _result = interpreter.interpret(&program);
    assert!(_result.is_ok());

    // Test code generation
    let context = Context::create();
    let mut codegen = CodeGenerator::new(&context, "test_module");
    let _result = codegen.compile(&program);
    assert!(_result.is_ok());
}

#[test]
fn test_full_pipeline_function_definition() {
    let source = "def add(x, y): return x + y; result = add(3, 4); print(result);";

    // Test parser
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    match &program {
        Node::Program(prog) => {
            assert_eq!(prog.statements.len(), 3); // 1 function + 1 assignment + 1 print
        }
        _ => panic!("Expected program node"),
    }

    // Test interpreter
    let mut interpreter = Interpreter::new();
    let _result = interpreter.interpret(&program);
    assert!(_result.is_ok());

    // Test code generation
    let context = Context::create();
    let mut codegen = CodeGenerator::new(&context, "test_module");
    let _result = codegen.compile(&program);
    assert!(_result.is_ok());
}

#[test]
fn test_full_pipeline_complex_expressions() {
    let source = "
        x = 10;
        y = 20;
        z = (x + y) * 2 - 5;
        print(z);
    ";

    // Test parser
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    match &program {
        Node::Program(prog) => {
            assert_eq!(prog.statements.len(), 4); // 3 assignments + 1 print
        }
        _ => panic!("Expected program node"),
    }

    // Test interpreter
    let mut interpreter = Interpreter::new();
    let _result = interpreter.interpret(&program);
    assert!(_result.is_ok());

    // Test code generation
    let context = Context::create();
    let mut codegen = CodeGenerator::new(&context, "test_module");
    let _result = codegen.compile(&program);
    assert!(_result.is_ok());
}

#[test]
fn test_error_handling() {
    // Test parsing invalid syntax
    let invalid_source = "x = ;"; // Invalid syntax
    let lexer = Lexer::new(invalid_source);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    // Even with invalid syntax, parser should return a program node
    match &program {
        Node::Program(_) => {} // Expected
        _ => panic!("Expected program node even with errors"),
    }

    // Test interpreter with the invalid program
    let mut interpreter = Interpreter::new();
    let _result = interpreter.interpret(&program);
    // Interpreter might return an error, which is expected
    // We're just testing that it doesn't panic
}

#[test]
fn test_codegen_output_to_file() {
    let source = "x = 42; print(x);";

    // Parse the program
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    // Generate code
    let context = Context::create();
    let mut codegen = CodeGenerator::new(&context, "test_module");
    let _result = codegen.compile(&program);
    assert!(_result.is_ok());

    // Write IR to file
    let temp_file = NamedTempFile::new().expect("Failed to create temp file");
    let temp_path = temp_file.path().to_str().unwrap();

    let _result = codegen.write_ir_to_file(temp_path);
    assert!(_result.is_ok());

    // Check that file was created and has content
    let content = fs::read_to_string(temp_path).expect("Failed to read temp file");
    assert!(!content.is_empty());
}

#[test]
fn test_simple_python_file() {
    let file_path = "tests/python_files/simple.py";
    let source =
        fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Failed to read {file_path}"));

    // Test lexer
    let mut lexer = Lexer::new(&source);
    let tokens: Vec<_> = std::iter::from_fn(|| {
        let token = lexer.next_token();
        if token != Token::Eof {
            Some(token)
        } else {
            None
        }
    })
    .collect();

    assert!(!tokens.is_empty(), "Failed to tokenize {file_path}");

    // Test parser
    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    match &program {
        Node::Program(prog) => {
            assert!(!prog.statements.is_empty(), "Failed to parse {file_path}");
        }
        _ => panic!("Expected program node for {file_path}"),
    }

    // Test interpreter with output capture
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&program);
    assert!(result.is_ok(), "Failed to interpret {file_path}");

    // Check that the interpreter produced the expected output
    // For simple.py, we expect it to print "42"
    let output = interpreter.get_output();
    assert_eq!(output, "42", "Interpreter output mismatch for {file_path}");

    // Test code generation
    let context = Context::create();
    let mut codegen = CodeGenerator::new(&context, "test_module");
    let _result = codegen.compile(&program);
    assert!(_result.is_ok(), "Failed to compile {file_path}");
}

#[test]
fn test_arithmetic_python_file() {
    let file_path = "tests/python_files/arithmetic.py";
    let source =
        fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Failed to read {file_path}"));

    // Test lexer
    let mut lexer = Lexer::new(&source);
    let tokens: Vec<_> = std::iter::from_fn(|| {
        let token = lexer.next_token();
        if token != Token::Eof {
            Some(token)
        } else {
            None
        }
    })
    .collect();

    assert!(!tokens.is_empty(), "Failed to tokenize {file_path}");

    // Test parser
    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    match &program {
        Node::Program(prog) => {
            assert!(!prog.statements.is_empty(), "Failed to parse {file_path}");
        }
        _ => panic!("Expected program node for {file_path}"),
    }

    // Test interpreter with output capture
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&program);
    assert!(result.is_ok(), "Failed to interpret {file_path}");

    // Check that the interpreter produced the expected output
    // For arithmetic.py, we expect it to print "20"
    let output = interpreter.get_output();
    assert_eq!(output, "20", "Interpreter output mismatch for {file_path}");

    // Test code generation
    let context = Context::create();
    let mut codegen = CodeGenerator::new(&context, "test_module");
    let _result = codegen.compile(&program);
    assert!(_result.is_ok(), "Failed to compile {file_path}");
}

#[test]
fn test_function_python_file() {
    let file_path = "tests/python_files/function.py";
    let source =
        fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Failed to read {file_path}"));

    // Test lexer
    let mut lexer = Lexer::new(&source);
    let tokens: Vec<_> = std::iter::from_fn(|| {
        let token = lexer.next_token();
        if token != Token::Eof {
            Some(token)
        } else {
            None
        }
    })
    .collect();

    assert!(!tokens.is_empty(), "Failed to tokenize {file_path}");

    // Test parser
    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    match &program {
        Node::Program(prog) => {
            assert!(!prog.statements.is_empty(), "Failed to parse {file_path}");
        }
        _ => panic!("Expected program node for {file_path}"),
    }

    // Test interpreter with output capture
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&program);
    assert!(result.is_ok(), "Failed to interpret {file_path}");

    // Check that the interpreter produced the expected output
    // For function.py, we expect it to print "8"
    let output = interpreter.get_output();
    assert_eq!(output, "8", "Interpreter output mismatch for {file_path}");

    // Test code generation
    let context = Context::create();
    let mut codegen = CodeGenerator::new(&context, "test_module");
    let _result = codegen.compile(&program);
    assert!(_result.is_ok(), "Failed to compile {file_path}");
}

#[test]
fn test_complex_python_file() {
    let file_path = "tests/python_files/complex.py";
    let source =
        fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Failed to read {file_path}"));

    // Test lexer
    let mut lexer = Lexer::new(&source);
    let tokens: Vec<_> = std::iter::from_fn(|| {
        let token = lexer.next_token();
        if token != Token::Eof {
            Some(token)
        } else {
            None
        }
    })
    .collect();

    assert!(!tokens.is_empty(), "Failed to tokenize {file_path}");

    // Test parser
    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    match &program {
        Node::Program(prog) => {
            assert!(!prog.statements.is_empty(), "Failed to parse {file_path}");
        }
        _ => panic!("Expected program node for {file_path}"),
    }

    // Test interpreter with output capture
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&program);
    assert!(result.is_ok(), "Failed to interpret {file_path}");

    // Check that the interpreter produced the expected output
    // For complex.py, we expect it to print "55"
    let output = interpreter.get_output();
    assert_eq!(output, "55", "Interpreter output mismatch for {file_path}");

    // Test code generation
    let context = Context::create();
    let mut codegen = CodeGenerator::new(&context, "test_module");
    let _result = codegen.compile(&program);
    assert!(_result.is_ok(), "Failed to compile {file_path}");
}

#[test]
fn test_arithmetic_operators_python_file() {
    let file_path = "tests/python_files/arithmetic_operators.py";
    let source =
        fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Failed to read {file_path}"));

    // Test lexer
    let mut lexer = Lexer::new(&source);
    let tokens: Vec<_> = std::iter::from_fn(|| {
        let token = lexer.next_token();
        if token != Token::Eof {
            Some(token)
        } else {
            None
        }
    })
    .collect();

    assert!(!tokens.is_empty(), "Failed to tokenize {file_path}");

    // Test parser
    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    match &program {
        Node::Program(prog) => {
            assert!(!prog.statements.is_empty(), "Failed to parse {file_path}");
        }
        _ => panic!("Expected program node for {file_path}"),
    }

    // Test interpreter with output capture
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&program);
    assert!(result.is_ok(), "Failed to interpret {file_path}");

    // Check that the interpreter produced the expected output
    // For arithmetic_operators.py, we expect it to print arithmetic operation results
    let output = interpreter.get_output();
    let expected_output = "Arithmetic Operations:\n10 + 3 = 13\n10 - 3 = 7\n10 * 3 = 30\n10 / 3 = 3.3333333333333335\n10 // 3 = 3\n10 % 3 = 1\n10 ** 3 = 1000\n\nUnary Operations:\n+10 = 10\n-10 = -10";
    assert_eq!(
        output, expected_output,
        "Interpreter output mismatch for {file_path}"
    );

    // Test code generation
    let context = Context::create();
    let mut codegen = CodeGenerator::new(&context, "test_module");
    let _result = codegen.compile(&program);
    assert!(_result.is_ok(), "Failed to compile {file_path}");
}

#[test]
fn test_comparison_operators_python_file() {
    let file_path = "tests/python_files/comparison_operators.py";
    let source =
        fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Failed to read {file_path}"));

    // Test lexer
    let mut lexer = Lexer::new(&source);
    let tokens: Vec<_> = std::iter::from_fn(|| {
        let token = lexer.next_token();
        if token != Token::Eof {
            Some(token)
        } else {
            None
        }
    })
    .collect();

    assert!(!tokens.is_empty(), "Failed to tokenize {file_path}");

    // Test parser
    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    match &program {
        Node::Program(prog) => {
            assert!(!prog.statements.is_empty(), "Failed to parse {file_path}");
        }
        _ => panic!("Expected program node for {file_path}"),
    }

    // Test interpreter with output capture
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&program);
    assert!(
        result.is_ok(),
        "Failed to interpret {file_path}: {:?}",
        result.err()
    );

    // Check that the interpreter produced the expected output
    // For comparison_operators.py, we expect it to print comparison operation results
    let output = interpreter.get_output();
    let expected_output = "Comparison Operations:\n10 == 10 = True\n10 != 5 = True\n5 < 10 = True\n10 > 5 = True\n5 <= 10 = True\n10 >= 10 = True\n\nString Comparisons:\n'hello' == 'hello' = True\n'hello' != 'world' = True\n\nBoolean Operations:\nTrue and False = False\nTrue or False = True\nnot True = False";
    assert_eq!(
        output, expected_output,
        "Interpreter output mismatch for {file_path}"
    );

    // Test code generation
    let context = Context::create();
    let mut codegen = CodeGenerator::new(&context, "test_module");
    let _result = codegen.compile(&program);
    assert!(_result.is_ok(), "Failed to compile {file_path}");
}

#[test]
fn test_logical_operators_python_file() {
    let file_path = "tests/python_files/logical_operators.py";
    let source =
        fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Failed to read {file_path}"));

    // Test lexer
    let mut lexer = Lexer::new(&source);
    let tokens: Vec<_> = std::iter::from_fn(|| {
        let token = lexer.next_token();
        if token != Token::Eof {
            Some(token)
        } else {
            None
        }
    })
    .collect();

    assert!(!tokens.is_empty(), "Failed to tokenize {file_path}");

    // Test parser
    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    match &program {
        Node::Program(prog) => {
            assert!(!prog.statements.is_empty(), "Failed to parse {file_path}");
        }
        _ => panic!("Expected program node for {file_path}"),
    }

    // Test interpreter with output capture
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&program);
    assert!(result.is_ok(), "Failed to interpret {file_path}");

    // Check that the interpreter produced the expected output
    // For logical_operators.py, we expect it to print logical operation results
    let output = interpreter.get_output();
    let expected_output = "Logical Operations:\nTrue and False = False\nTrue or False = True\nnot True = False\n\nComplex Logical Operations:\n(True and True) and (False and False) = False\n(True or False) or (True or False) = True\nnot (True and False) = True\n\nLogical Operations with Non-boolean Values:\n10 and 0 = 0\n10 or 0 = 10\nnot 'hello' = False\n\nShort-circuit Evaluation:\nTesting 'False and true_func()':\nfalse_func called\nTesting 'True or false_func()':\nResult of 'False and true_func()': False\nResult of 'True or false_func()': True";
    assert_eq!(
        output, expected_output,
        "Interpreter output mismatch for {file_path}"
    );

    // Test code generation
    let context = Context::create();
    let mut codegen = CodeGenerator::new(&context, "test_module");
    let _result = codegen.compile(&program);
    assert!(_result.is_ok(), "Failed to compile {file_path}");
}

#[test]
fn test_bitwise_operators_python_file() {
    let file_path = "tests/python_files/bitwise_operators.py";
    let source =
        fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Failed to read {file_path}"));

    // Test lexer
    let mut lexer = Lexer::new(&source);
    let tokens: Vec<_> = std::iter::from_fn(|| {
        let token = lexer.next_token();
        if token != Token::Eof {
            Some(token)
        } else {
            None
        }
    })
    .collect();

    assert!(!tokens.is_empty(), "Failed to tokenize {file_path}");

    // Test parser
    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    match &program {
        Node::Program(prog) => {
            assert!(!prog.statements.is_empty(), "Failed to parse {file_path}");
        }
        _ => panic!("Expected program node for {file_path}"),
    }

    // Test interpreter with output capture
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&program);
    assert!(result.is_ok(), "Failed to interpret {file_path}");

    // Check that the interpreter produced the expected output
    // For bitwise_operators.py, we expect it to print bitwise operation results
    let output = interpreter.get_output();
    let expected_output = "Bitwise Operations:\n12 & 10 = 8 (binary: 0b1000)\n12 | 10 = 14 (binary: 0b1110)\n12 ^ 10 = 6 (binary: 0b110)\n~12 = -13 (binary: -0b1101)\n12 << 2 = 48 (binary: 0b110000)\n12 >> 2 = 3 (binary: 0b11)\n\nAdditional Bitwise Operations:\n15 & 7 = 7 (binary: 0b111)\n15 | 7 = 15 (binary: 0b1111)\n15 ^ 7 = 8 (binary: 0b1000)\n~7 = -8 (binary: -0b1000)\n\nBitwise Operations with Negative Numbers:\n-12 & -10 = -16\n-12 | -10 = -2\n-12 ^ -10 = 14\n~-12 = 11\n\nBit Shifting with Negative Numbers:\n-12 << 2 = -48\n-12 >> 2 = -3";
    assert_eq!(
        output, expected_output,
        "Interpreter output mismatch for {file_path}"
    );

    // Test code generation
    let context = Context::create();
    let mut codegen = CodeGenerator::new(&context, "test_module");
    let _result = codegen.compile(&program);
    assert!(_result.is_ok(), "Failed to compile {file_path}");
}

#[test]
fn test_assignment_operators_python_file() {
    let file_path = "tests/python_files/assignment_operators.py";
    let source =
        fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Failed to read {file_path}"));

    // Test lexer
    let mut lexer = Lexer::new(&source);
    let tokens: Vec<_> = std::iter::from_fn(|| {
        let token = lexer.next_token();
        if token != Token::Eof {
            Some(token)
        } else {
            None
        }
    })
    .collect();

    assert!(!tokens.is_empty(), "Failed to tokenize {file_path}");

    // Test parser
    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    match &program {
        Node::Program(prog) => {
            assert!(!prog.statements.is_empty(), "Failed to parse {file_path}");
        }
        _ => panic!("Expected program node for {file_path}"),
    }

    // Test interpreter with output capture
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&program);
    assert!(result.is_ok(), "Failed to interpret {file_path}");

    // Check that the interpreter produced the expected output
    // For assignment_operators.py, we expect it to print assignment operation results
    let output = interpreter.get_output();
    let expected_output = "Initial values:\na = 20\nb = 10\n\nAfter a += 10: a = 30\nAfter a -= 5: a = 25\nAfter a *= 2: a = 50\nAfter a /= 3: a = 16.666666666666668\nAfter a //= 2: a = 8.0\nAfter a %= 3: a = 2.0\nAfter a **= 2: a = 4.0\n\nReset a to 12 (binary: 0b1100)\nAfter a &= 10: a = 8 (binary: 0b1000)\nAfter a |= 4: a = 12 (binary: 0b1100)\nAfter a ^= 2: a = 14 (binary: 0b1110)\nAfter a <<= 1: a = 28 (binary: 0b11100)\nAfter a >>= 1: a = 14 (binary: 0b1110)\n\nMultiple assignment: x = y = z = 100\nChained assignment: p = q = r = 50\n\nString augmented assignment: s += ' World' => s = 'Hello World'\nList augmented assignment: t += [4, 5] => t = [1, 2, 3, 4, 5]";
    assert_eq!(
        output, expected_output,
        "Interpreter output mismatch for {file_path}"
    );

    // Test code generation
    let context = Context::create();
    let mut codegen = CodeGenerator::new(&context, "test_module");
    let _result = codegen.compile(&program);
    assert!(_result.is_ok(), "Failed to compile {file_path}");
}

#[test]
fn test_comments_python_file() {
    let file_path = "tests/python_files/comments.py";
    let source =
        fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Failed to read {file_path}"));
    let expected_output = "42";

    // Test lexer
    let mut lexer = Lexer::new(&source);
    let tokens: Vec<_> = std::iter::from_fn(|| {
        let token = lexer.next_token();
        if token != Token::Eof {
            Some(token)
        } else {
            None
        }
    })
    .collect();

    assert!(!tokens.is_empty(), "Failed to tokenize {file_path}");

    // Test parser
    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    match &program {
        Node::Program(prog) => {
            assert!(!prog.statements.is_empty(), "Failed to parse {file_path}");
        }
        _ => panic!("Expected program node for {file_path}"),
    }

    // Test interpreter with output capture
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&program);
    assert!(result.is_ok(), "Failed to interpret {file_path}");

    let output = interpreter.get_output();
    assert_eq!(
        output, expected_output,
        "Interpreter output mismatch for {file_path}\nExpected: {expected_output}\nGot: {output}"
    );
}
