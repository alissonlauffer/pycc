use pycc::ast::*;
use pycc::lexer::Lexer;
use pycc::parser::Parser;

#[test]
fn test_parse_integer_literal() {
    let input = "5;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    match program {
        Node::Program(prog) => {
            assert_eq!(prog.statements.len(), 1);
            match &prog.statements[0] {
                Node::ExpressionStatement(expr_stmt) => match &*expr_stmt.expression {
                    Node::Literal(literal) => match &literal.value {
                        LiteralValue::Integer(value) => assert_eq!(*value, 5),
                        _ => panic!("Expected integer literal"),
                    },
                    _ => panic!("Expected literal expression"),
                },
                _ => panic!("Expected expression statement"),
            }
        }
        _ => panic!("Expected program node"),
    }
}

#[test]
fn test_parse_float_literal() {
    let input = "3.14;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    match program {
        Node::Program(prog) => {
            assert_eq!(prog.statements.len(), 1);
            match &prog.statements[0] {
                Node::ExpressionStatement(expr_stmt) => match &*expr_stmt.expression {
                    Node::Literal(literal) => match &literal.value {
                        LiteralValue::Float(value) => assert_eq!(*value, 3.14),
                        _ => panic!("Expected float literal"),
                    },
                    _ => panic!("Expected literal expression"),
                },
                _ => panic!("Expected expression statement"),
            }
        }
        _ => panic!("Expected program node"),
    }
}

#[test]
fn test_parse_string_literal() {
    let input = "\"hello\";";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    match program {
        Node::Program(prog) => {
            assert_eq!(prog.statements.len(), 1);
            match &prog.statements[0] {
                Node::ExpressionStatement(expr_stmt) => match &*expr_stmt.expression {
                    Node::Literal(literal) => match &literal.value {
                        LiteralValue::String(value) => assert_eq!(value, "hello"),
                        _ => panic!("Expected string literal"),
                    },
                    _ => panic!("Expected literal expression"),
                },
                _ => panic!("Expected expression statement"),
            }
        }
        _ => panic!("Expected program node"),
    }
}

#[test]
fn test_parse_boolean_literal() {
    let input = "True;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    match program {
        Node::Program(prog) => {
            assert_eq!(prog.statements.len(), 1);
            match &prog.statements[0] {
                Node::ExpressionStatement(expr_stmt) => match &*expr_stmt.expression {
                    Node::Literal(literal) => match &literal.value {
                        LiteralValue::Boolean(value) => assert!(*value),
                        _ => panic!("Expected boolean literal"),
                    },
                    _ => panic!("Expected literal expression"),
                },
                _ => panic!("Expected expression statement"),
            }
        }
        _ => panic!("Expected program node"),
    }
}

#[test]
fn test_parse_none_literal() {
    let input = "None;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    match program {
        Node::Program(prog) => {
            assert_eq!(prog.statements.len(), 1);
            match &prog.statements[0] {
                Node::ExpressionStatement(expr_stmt) => {
                    match &*expr_stmt.expression {
                        Node::Literal(literal) => {
                            match &literal.value {
                                LiteralValue::None => {} // Success
                                _ => panic!("Expected None literal"),
                            }
                        }
                        _ => panic!("Expected literal expression"),
                    }
                }
                _ => panic!("Expected expression statement"),
            }
        }
        _ => panic!("Expected program node"),
    }
}

#[test]
fn test_parse_identifier() {
    let input = "x;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    match program {
        Node::Program(prog) => {
            assert_eq!(prog.statements.len(), 1);
            match &prog.statements[0] {
                Node::ExpressionStatement(expr_stmt) => match &*expr_stmt.expression {
                    Node::Identifier(identifier) => {
                        assert_eq!(identifier.name, "x");
                    }
                    _ => panic!("Expected identifier expression"),
                },
                _ => panic!("Expected expression statement"),
            }
        }
        _ => panic!("Expected program node"),
    }
}

#[test]
fn test_parse_binary_expressions() {
    let input = "5 + 3;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    match program {
        Node::Program(prog) => {
            assert_eq!(prog.statements.len(), 1);
            match &prog.statements[0] {
                Node::ExpressionStatement(expr_stmt) => match &*expr_stmt.expression {
                    Node::Binary(binary) => {
                        match &*binary.left {
                            Node::Literal(literal) => match &literal.value {
                                LiteralValue::Integer(value) => assert_eq!(*value, 5),
                                _ => panic!("Expected integer literal"),
                            },
                            _ => panic!("Expected literal expression"),
                        }

                        assert_eq!(binary.operator, BinaryOperator::Add);

                        match &*binary.right {
                            Node::Literal(literal) => match &literal.value {
                                LiteralValue::Integer(value) => assert_eq!(*value, 3),
                                _ => panic!("Expected integer literal"),
                            },
                            _ => panic!("Expected literal expression"),
                        }
                    }
                    _ => panic!("Expected binary expression"),
                },
                _ => panic!("Expected expression statement"),
            }
        }
        _ => panic!("Expected program node"),
    }
}

#[test]
fn test_parse_assignment() {
    let input = "x = 42;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    match program {
        Node::Program(prog) => {
            assert_eq!(prog.statements.len(), 1);
            match &prog.statements[0] {
                Node::Assignment(assignment) => {
                    assert_eq!(assignment.name, "x");
                    match &*assignment.value {
                        Node::Literal(literal) => match &literal.value {
                            LiteralValue::Integer(value) => assert_eq!(*value, 42),
                            _ => panic!("Expected integer literal"),
                        },
                        _ => panic!("Expected literal expression"),
                    }
                }
                _ => panic!("Expected assignment statement"),
            }
        }
        _ => panic!("Expected program node"),
    }
}

#[test]
fn test_parse_function_definition() {
    let input = "def add(x, y): return x + y;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    match program {
        Node::Program(prog) => {
            assert_eq!(prog.statements.len(), 1);
            match &prog.statements[0] {
                Node::Function(function) => {
                    assert_eq!(function.name, "add");
                    assert_eq!(function.parameters.len(), 2);
                    assert_eq!(function.parameters[0], "x");
                    assert_eq!(function.parameters[1], "y");

                    // Check function body
                    match &*function.body {
                        Node::Return(return_stmt) => {
                            if let Some(value) = &return_stmt.value {
                                match &**value {
                                    Node::Binary(binary) => {
                                        // Check left operand (x)
                                        match &*binary.left {
                                            Node::Identifier(identifier) => {
                                                assert_eq!(identifier.name, "x");
                                            }
                                            _ => panic!("Expected identifier"),
                                        }

                                        // Check operator
                                        assert_eq!(binary.operator, BinaryOperator::Add);

                                        // Check right operand (y)
                                        match &*binary.right {
                                            Node::Identifier(identifier) => {
                                                assert_eq!(identifier.name, "y");
                                            }
                                            _ => panic!("Expected identifier"),
                                        }
                                    }
                                    _ => panic!("Expected binary expression"),
                                }
                            } else {
                                panic!("Expected return value");
                            }
                        }
                        _ => panic!("Expected return statement"),
                    }
                }
                _ => panic!("Expected function definition"),
            }
        }
        _ => panic!("Expected program node"),
    }
}

#[test]
fn test_parse_function_call() {
    let input = "print(\"Hello, World!\");";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    match program {
        Node::Program(prog) => {
            assert_eq!(prog.statements.len(), 1);
            match &prog.statements[0] {
                Node::ExpressionStatement(expr_stmt) => match &*expr_stmt.expression {
                    Node::Call(call) => {
                        assert_eq!(call.callee, "print");
                        assert_eq!(call.arguments.len(), 1);

                        match &call.arguments[0] {
                            Node::Literal(literal) => match &literal.value {
                                LiteralValue::String(value) => assert_eq!(value, "Hello, World!"),
                                _ => panic!("Expected string literal"),
                            },
                            _ => panic!("Expected literal argument"),
                        }
                    }
                    _ => panic!("Expected function call"),
                },
                _ => panic!("Expected expression statement"),
            }
        }
        _ => panic!("Expected program node"),
    }
}

#[test]
fn test_parse_complex_expression() {
    let input = "x = 5 + 3 * 2;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    match program {
        Node::Program(prog) => {
            assert_eq!(prog.statements.len(), 1);
            match &prog.statements[0] {
                Node::Assignment(assignment) => {
                    assert_eq!(assignment.name, "x");

                    // Check the expression: 5 + 3 * 2
                    match &*assignment.value {
                        Node::Binary(binary) => {
                            // Should be (5 + (3 * 2)) due to operator precedence
                            assert_eq!(binary.operator, BinaryOperator::Add);

                            // Check left operand (5)
                            match &*binary.left {
                                Node::Literal(literal) => match &literal.value {
                                    LiteralValue::Integer(value) => assert_eq!(*value, 5),
                                    _ => panic!("Expected integer literal"),
                                },
                                _ => panic!("Expected literal expression"),
                            }

                            // Check right operand (3 * 2)
                            match &*binary.right {
                                Node::Binary(inner_binary) => {
                                    assert_eq!(inner_binary.operator, BinaryOperator::Multiply);

                                    // Check left operand of inner binary (3)
                                    match &*inner_binary.left {
                                        Node::Literal(literal) => match &literal.value {
                                            LiteralValue::Integer(value) => assert_eq!(*value, 3),
                                            _ => panic!("Expected integer literal"),
                                        },
                                        _ => panic!("Expected literal expression"),
                                    }

                                    // Check right operand of inner binary (2)
                                    match &*inner_binary.right {
                                        Node::Literal(literal) => match &literal.value {
                                            LiteralValue::Integer(value) => assert_eq!(*value, 2),
                                            _ => panic!("Expected integer literal"),
                                        },
                                        _ => panic!("Expected literal expression"),
                                    }
                                }
                                _ => panic!("Expected binary expression"),
                            }
                        }
                        _ => panic!("Expected binary expression"),
                    }
                }
                _ => panic!("Expected assignment statement"),
            }
        }
        _ => panic!("Expected program node"),
    }
}
