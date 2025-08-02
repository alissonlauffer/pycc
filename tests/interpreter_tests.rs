use pycc::interpreter::Interpreter;
use pycc::lexer::Lexer;
use pycc::parser::Parser;

#[test]
fn test_interpret_integer_literal() {
    let input = "42;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&program);

    assert!(result.is_ok());
}

#[test]
fn test_interpret_float_literal() {
    let input = "3.14;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&program);

    assert!(result.is_ok());
}

#[test]
fn test_interpret_string_literal() {
    let input = "\"hello\";";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&program);

    assert!(result.is_ok());
}

#[test]
fn test_interpret_boolean_literal() {
    let input = "True;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&program);

    assert!(result.is_ok());
}

#[test]
fn test_interpret_none_literal() {
    let input = "None;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&program);

    assert!(result.is_ok());
}

#[test]
fn test_interpret_variable_assignment() {
    let input = "x = 42;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&program);

    assert!(result.is_ok());
}

#[test]
fn test_interpret_variable_reference() {
    let input = "x = 42; x;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&program);

    assert!(result.is_ok());
}

#[test]
fn test_interpret_binary_operations() {
    let tests = vec![
        ("5 + 3;", 8),
        ("10 - 4;", 6),
        ("6 * 7;", 42),
        ("15 / 3;", 5),
    ];

    for (input, expected) in tests {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        let mut interpreter = Interpreter::new();
        let result = interpreter.interpret(&program);

        assert!(result.is_ok());
    }
}

#[test]
fn test_interpret_function_definition() {
    let input = "def add(x, y): return x + y;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&program);

    assert!(result.is_ok());
}

#[test]
fn test_interpret_print_function() {
    let input = "print(\"Hello, World!\");";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&program);

    assert!(result.is_ok());
}

#[test]
fn test_interpret_complex_expression() {
    let input = "x = 5 + 3 * 2;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&program);

    assert!(result.is_ok());
}

#[test]
fn test_interpret_function_call() {
    let input = "def add(a, b): return a + b; result = add(5, 3);";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&program);

    assert!(result.is_ok());
}
