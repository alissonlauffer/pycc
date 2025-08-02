use inkwell::context::Context;
use pycc::codegen::CodeGenerator;
use pycc::lexer::Lexer;
use pycc::parser::Parser;

#[test]
fn test_codegen_integer_literal() {
    let input = "42;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    let context = Context::create();
    let mut codegen = CodeGenerator::new(&context, "test_module");
    let result = codegen.compile(&program);

    assert!(result.is_ok());
}

#[test]
fn test_codegen_float_literal() {
    let input = "3.14;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    let context = Context::create();
    let mut codegen = CodeGenerator::new(&context, "test_module");
    let result = codegen.compile(&program);

    assert!(result.is_ok());
}

#[test]
fn test_codegen_string_literal() {
    let input = "\"hello\";";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    let context = Context::create();
    let mut codegen = CodeGenerator::new(&context, "test_module");
    let result = codegen.compile(&program);

    assert!(result.is_ok());
}

#[test]
fn test_codegen_boolean_literal() {
    let input = "True;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    let context = Context::create();
    let mut codegen = CodeGenerator::new(&context, "test_module");
    let result = codegen.compile(&program);

    assert!(result.is_ok());
}

#[test]
fn test_codegen_variable_assignment() {
    let input = "x = 42;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    let context = Context::create();
    let mut codegen = CodeGenerator::new(&context, "test_module");
    let result = codegen.compile(&program);

    assert!(result.is_ok());
}

#[test]
fn test_codegen_binary_operations() {
    let tests = vec!["5 + 3;", "10 - 4;", "6 * 7;", "15 / 3;"];

    for input in tests {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        let context = Context::create();
        let mut codegen = CodeGenerator::new(&context, "test_module");
        let result = codegen.compile(&program);

        assert!(result.is_ok());
    }
}

#[test]
fn test_codegen_function_definition() {
    let input = "def add(x, y): return x + y;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    let context = Context::create();
    let mut codegen = CodeGenerator::new(&context, "test_module");
    let result = codegen.compile(&program);

    assert!(result.is_ok());
}

#[test]
fn test_codegen_print_function() {
    let input = "print(\"Hello, World!\");";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    let context = Context::create();
    let mut codegen = CodeGenerator::new(&context, "test_module");
    let result = codegen.compile(&program);

    assert!(result.is_ok());
}

#[test]
fn test_codegen_complex_expression() {
    let input = "x = 5 + 3 * 2;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    let context = Context::create();
    let mut codegen = CodeGenerator::new(&context, "test_module");
    let result = codegen.compile(&program);

    assert!(result.is_ok());
}

#[test]
fn test_codegen_function_call() {
    let input = "def add(a, b): return a + b; result = add(5, 3);";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    let context = Context::create();
    let mut codegen = CodeGenerator::new(&context, "test_module");
    let result = codegen.compile(&program);

    assert!(result.is_ok());
}

#[test]
fn test_codegen_print_ir() {
    let input = "x = 42; print(x);";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    let context = Context::create();
    let mut codegen = CodeGenerator::new(&context, "test_module");
    let result = codegen.compile(&program);

    assert!(result.is_ok());

    // Test that we can print the IR without panicking
    codegen.print_ir();
}
