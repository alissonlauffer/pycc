use inkwell::context::Context;
use pycc::ast::*;
use pycc::codegen::CodeGenerator;
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

    // Test code generation with the invalid program
    let context = Context::create();
    let mut codegen = CodeGenerator::new(&context, "test_module");
    let _result = codegen.compile(&program);
    // Code generation might return an error, which is expected
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

    // Test code generation
    let context = Context::create();
    let mut codegen = CodeGenerator::new(&context, "test_module");
    let _result = codegen.compile(&program);
    assert!(_result.is_ok(), "Failed to compile {file_path}");
}
