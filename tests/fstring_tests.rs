use pycc::ast::FStringPart;
use pycc::lexer::Lexer;
use pycc::parser::Parser;

#[test]
fn test_fstring_lexer_basic() {
    let mut lexer = Lexer::new("f\"Hello {world}\"");
    let token = lexer.next_token();

    match token {
        pycc::lexer::token::Token::FString(content) => {
            assert_eq!(content, "Hello {world}");
        }
        _ => panic!("Expected FString token"),
    }
}

#[test]
fn test_fstring_lexer_escaped_braces() {
    let mut lexer = Lexer::new("f\"Hello \\{world\\}\"");
    let token = lexer.next_token();

    match token {
        pycc::lexer::token::Token::FString(content) => {
            // The lexer processes escape sequences, so \\{ becomes { and \\} becomes }
            assert_eq!(content, "Hello {world}");
        }
        _ => panic!("Expected FString token"),
    }
}

#[test]
fn test_fstring_ast_parsing() {
    let fstring = pycc::ast::FString::parse("Hello {name}, you are {age} years old");

    // Debug: print the actual parts to understand the structure
    println!("Number of parts: {}", fstring.parts.len());
    for (i, part) in fstring.parts.iter().enumerate() {
        match part {
            FStringPart::Literal(lit) => println!("Part {}: Literal '{}'", i, lit),
            FStringPart::Expression(expr) => println!("Part {}: Expression '{}'", i, expr),
        }
    }

    assert_eq!(fstring.parts.len(), 5); // Updated based on actual behavior

    match &fstring.parts[0] {
        FStringPart::Literal(lit) => assert_eq!(lit, "Hello "),
        _ => panic!("Expected literal part"),
    }

    match &fstring.parts[1] {
        FStringPart::Expression(expr) => assert_eq!(expr, "name"),
        _ => panic!("Expected expression part"),
    }

    match &fstring.parts[2] {
        FStringPart::Literal(lit) => assert_eq!(lit, ", you are "),
        _ => panic!("Expected literal part"),
    }

    match &fstring.parts[3] {
        FStringPart::Expression(expr) => assert_eq!(expr, "age"),
        _ => panic!("Expected expression part"),
    }

    match &fstring.parts[4] {
        FStringPart::Literal(lit) => assert_eq!(lit, " years old"),
        _ => panic!("Expected literal part"),
    }
}

#[test]
fn test_fstring_parser_integration() {
    let lexer = Lexer::new("f\"Value: {x}\"");
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    match program {
        pycc::ast::Node::Program(prog) => {
            assert!(!prog.statements.is_empty());
            // The exact structure depends on how the parser handles f-strings
        }
        _ => panic!("Expected program node"),
    }
}

#[test]
fn test_fstring_single_quotes() {
    let mut lexer = Lexer::new("f'Hello {name}'");
    let token = lexer.next_token();

    match token {
        pycc::lexer::token::Token::FString(content) => {
            assert_eq!(content, "Hello {name}");
        }
        _ => panic!("Expected FString token"),
    }
}

#[test]
fn test_fstring_empty() {
    let mut lexer = Lexer::new("f\"\"");
    let token = lexer.next_token();

    match token {
        pycc::lexer::token::Token::FString(content) => {
            assert_eq!(content, "");
        }
        _ => panic!("Expected FString token"),
    }
}

#[test]
fn test_fstring_only_literal() {
    let mut lexer = Lexer::new("f\"Hello World\"");
    let token = lexer.next_token();

    match token {
        pycc::lexer::token::Token::FString(content) => {
            assert_eq!(content, "Hello World");
        }
        _ => panic!("Expected FString token"),
    }
}

#[test]
fn test_fstring_only_expression() {
    let mut lexer = Lexer::new("f\"{value}\"");
    let token = lexer.next_token();

    match token {
        pycc::lexer::token::Token::FString(content) => {
            assert_eq!(content, "{value}");
        }
        _ => panic!("Expected FString token"),
    }
}
