use pycc::lexer::{Lexer, Token};

#[test]
fn test_single_character_tokens() {
    let input = "=;:,(){}+-*/";
    let mut lexer = Lexer::new(input);

    let expected_tokens = vec![
        Token::Assign,
        Token::Semicolon,
        Token::Colon,
        Token::Comma,
        Token::LeftParen,
        Token::RightParen,
        Token::LeftBrace,
        Token::RightBrace,
        Token::Plus,
        Token::Minus,
        Token::Multiply,
        Token::Divide,
        Token::Eof,
    ];

    for expected in expected_tokens {
        let token = lexer.next_token();
        assert_eq!(token, expected, "Expected {expected:?}, got {token:?}");
    }
}

#[test]
fn test_multi_character_tokens() {
    let input = "== != <= >= **";
    let mut lexer = Lexer::new(input);

    let expected_tokens = vec![
        Token::Equal,
        Token::NotEqual,
        Token::LessEqual,
        Token::GreaterEqual,
        Token::Power,
        Token::Eof,
    ];

    for expected in expected_tokens {
        let token = lexer.next_token();
        assert_eq!(token, expected, "Expected {expected:?}, got {token:?}");
    }
}

#[test]
fn test_identifiers_and_keywords() {
    let input = "def if else while return True False None and or not x y123 _test";
    let mut lexer = Lexer::new(input);

    let expected_tokens = vec![
        Token::Def,
        Token::If,
        Token::Else,
        Token::While,
        Token::Return,
        Token::Boolean(true),
        Token::Boolean(false),
        Token::None,
        Token::And,
        Token::Or,
        Token::Not,
        Token::Identifier("x".to_string()),
        Token::Identifier("y123".to_string()),
        Token::Identifier("_test".to_string()),
        Token::Eof,
    ];

    for expected in expected_tokens {
        let token = lexer.next_token();
        assert_eq!(token, expected, "Expected {expected:?}, got {token:?}");
    }
}

#[test]
fn test_numbers() {
    let input = "42 3.14 0.5 100";
    let mut lexer = Lexer::new(input);

    let expected_tokens = vec![
        Token::Integer(42),
        Token::Float(3.14),
        Token::Float(0.5),
        Token::Integer(100),
        Token::Eof,
    ];

    for expected in expected_tokens {
        let token = lexer.next_token();
        assert_eq!(token, expected, "Expected {expected:?}, got {token:?}");
    }
}

#[test]
fn test_strings() {
    let input = "\"hello\" 'world' \"123\"";
    let mut lexer = Lexer::new(input);

    let expected_tokens = vec![
        Token::String("hello".to_string()),
        Token::String("world".to_string()),
        Token::String("123".to_string()),
        Token::Eof,
    ];

    for expected in expected_tokens {
        let token = lexer.next_token();
        assert_eq!(token, expected, "Expected {expected:?}, got {token:?}");
    }
}

#[test]
fn test_whitespace_handling() {
    let input = "  \n\t\r  x   =   42  ";
    let mut lexer = Lexer::new(input);

    let expected_tokens = vec![
        Token::Identifier("x".to_string()),
        Token::Assign,
        Token::Integer(42),
        Token::Eof,
    ];

    for expected in expected_tokens {
        let token = lexer.next_token();
        assert_eq!(token, expected, "Expected {expected:?}, got {token:?}");
    }
}

#[test]
fn test_complex_expressions() {
    let input = "x = 5 + 3 * 2";
    let mut lexer = Lexer::new(input);

    let expected_tokens = vec![
        Token::Identifier("x".to_string()),
        Token::Assign,
        Token::Integer(5),
        Token::Plus,
        Token::Integer(3),
        Token::Multiply,
        Token::Integer(2),
        Token::Eof,
    ];

    for expected in expected_tokens {
        let token = lexer.next_token();
        assert_eq!(token, expected, "Expected {expected:?}, got {token:?}");
    }
}

#[test]
fn test_function_definition() {
    let input = "def add(x, y): return x + y";
    let mut lexer = Lexer::new(input);

    let expected_tokens = vec![
        Token::Def,
        Token::Identifier("add".to_string()),
        Token::LeftParen,
        Token::Identifier("x".to_string()),
        Token::Comma,
        Token::Identifier("y".to_string()),
        Token::RightParen,
        Token::Colon,
        Token::Return,
        Token::Identifier("x".to_string()),
        Token::Plus,
        Token::Identifier("y".to_string()),
        Token::Eof,
    ];

    for expected in expected_tokens {
        let token = lexer.next_token();
        assert_eq!(token, expected, "Expected {expected:?}, got {token:?}");
    }
}

#[test]
fn test_comments() {
    let input = "# This is a comment\nx = 5 # Another comment";
    let mut lexer = Lexer::new(input);

    let expected_tokens = vec![
        Token::Comment(" This is a comment".to_string()),
        Token::Identifier("x".to_string()),
        Token::Assign,
        Token::Integer(5),
        Token::Comment(" Another comment".to_string()),
        Token::Eof,
    ];

    for expected in expected_tokens {
        let token = lexer.next_token();
        assert_eq!(token, expected, "Expected {expected:?}, got {token:?}");
    }
}

#[test]
fn test_escape_sequences() {
    let input = r#""hello\nworld" 'test\ttab' "quote\"inside" 'single\'quote'"#;
    let mut lexer = Lexer::new(input);

    let expected_tokens = vec![
        Token::String("hello\nworld".to_string()),
        Token::String("test\ttab".to_string()),
        Token::String("quote\"inside".to_string()),
        Token::String("single'quote".to_string()),
        Token::Eof,
    ];

    for expected in expected_tokens {
        let token = lexer.next_token();
        assert_eq!(token, expected, "Expected {expected:?}, got {token:?}");
    }
}

#[test]
fn test_backslash_escape() {
    let input = r#""backslash\\test""#;
    let mut lexer = Lexer::new(input);

    let expected_tokens = vec![Token::String("backslash\\test".to_string()), Token::Eof];

    for expected in expected_tokens {
        let token = lexer.next_token();
        assert_eq!(token, expected, "Expected {expected:?}, got {token:?}");
    }
}
