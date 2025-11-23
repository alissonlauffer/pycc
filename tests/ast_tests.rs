use pycc::ast::*;
use std::f64::consts::PI;

#[test]
fn test_program_creation() {
    let program = Program::new();
    assert_eq!(program.statements.len(), 0);
}

#[test]
fn test_literal_nodes() {
    // Test integer literal
    let int_literal = Node::Literal(Literal {
        value: LiteralValue::Integer(42),
    });

    // Test float literal
    let float_literal = Node::Literal(Literal {
        value: LiteralValue::Float(PI),
    });

    // Test string literal
    let _string_literal = Node::Literal(Literal {
        value: LiteralValue::String("hello".to_string()),
    });

    // Test boolean literal
    let _bool_literal = Node::Literal(Literal {
        value: LiteralValue::Boolean(true),
    });

    // Test None literal
    let _none_literal = Node::Literal(Literal {
        value: LiteralValue::None,
    });

    // Verify they are created correctly
    match int_literal {
        Node::Literal(lit) => match lit.value {
            LiteralValue::Integer(val) => assert_eq!(val, 42),
            _ => panic!("Expected integer literal"),
        },
        _ => panic!("Expected literal node"),
    }

    match float_literal {
        Node::Literal(lit) => match lit.value {
            LiteralValue::Float(val) => assert_eq!(val, PI),
            _ => panic!("Expected float literal"),
        },
        _ => panic!("Expected literal node"),
    }
}

#[test]
fn test_binary_operations() {
    let left = Box::new(Node::Literal(Literal {
        value: LiteralValue::Integer(5),
    }));
    let right = Box::new(Node::Literal(Literal {
        value: LiteralValue::Integer(3),
    }));

    let add_expr = Node::Binary(Binary {
        left: left.clone(),
        operator: BinaryOperator::Add,
        right: right.clone(),
    });

    let _sub_expr = Node::Binary(Binary {
        left: left.clone(),
        operator: BinaryOperator::Subtract,
        right: right.clone(),
    });

    let _mul_expr = Node::Binary(Binary {
        left: left.clone(),
        operator: BinaryOperator::Multiply,
        right: right.clone(),
    });

    let _div_expr = Node::Binary(Binary {
        left: left.clone(),
        operator: BinaryOperator::Divide,
        right: right.clone(),
    });

    // Verify binary operations are created correctly
    match add_expr {
        Node::Binary(bin) => {
            assert_eq!(bin.operator, BinaryOperator::Add);
            match *bin.left {
                Node::Literal(lit) => match lit.value {
                    LiteralValue::Integer(val) => assert_eq!(val, 5),
                    _ => panic!("Expected integer literal"),
                },
                _ => panic!("Expected literal node"),
            }
        }
        _ => panic!("Expected binary node"),
    }
}

#[test]
fn test_identifier_and_assignment() {
    let identifier = Node::Identifier(Identifier {
        name: "x".to_string(),
    });

    let assignment = Node::Assignment(Assignment {
        name: "x".to_string(),
        value: Box::new(Node::Literal(Literal {
            value: LiteralValue::Integer(42),
        })),
    });

    // Verify identifier
    match identifier {
        Node::Identifier(ident) => assert_eq!(ident.name, "x"),
        _ => panic!("Expected identifier node"),
    }

    // Verify assignment
    match assignment {
        Node::Assignment(assign) => {
            assert_eq!(assign.name, "x");
            match *assign.value {
                Node::Literal(lit) => match lit.value {
                    LiteralValue::Integer(val) => assert_eq!(val, 42),
                    _ => panic!("Expected integer literal"),
                },
                _ => panic!("Expected literal node"),
            }
        }
        _ => panic!("Expected assignment node"),
    }
}

#[test]
fn test_function_node() {
    let function = Node::Function(Function {
        name: "test_func".to_string(),
        parameters: vec!["a".to_string(), "b".to_string()],
        body: Box::new(Node::Return(Return {
            value: Some(Box::new(Node::Literal(Literal {
                value: LiteralValue::Integer(42),
            }))),
        })),
    });

    match function {
        Node::Function(func) => {
            assert_eq!(func.name, "test_func");
            assert_eq!(func.parameters.len(), 2);
            assert_eq!(func.parameters[0], "a");
            assert_eq!(func.parameters[1], "b");
        }
        _ => panic!("Expected function node"),
    }
}

#[test]
fn test_call_node() {
    let call = Node::Call(Call {
        callee: "print".to_string(),
        arguments: vec![Node::Literal(Literal {
            value: LiteralValue::String("Hello, World!".to_string()),
        })],
    });

    match call {
        Node::Call(c) => {
            assert_eq!(c.callee, "print");
            assert_eq!(c.arguments.len(), 1);
        }
        _ => panic!("Expected call node"),
    }
}
