#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Integer(i64),
    Float(f64),
    String(String),
    FString(String), // F-string literal
    Boolean(bool),
    None,

    // Identifiers
    Identifier(String),

    // Comments
    Comment(String),

    // Keywords
    Def,
    If,
    Else,
    While,
    Return,
    True,
    False,

    // Operators
    Plus,         // +
    Minus,        // -
    Multiply,     // *
    Divide,       // /
    FloorDivide,  // //
    Modulo,       // %
    Power,        // **
    Assign,       // =
    Equal,        // ==
    NotEqual,     // !=
    Less,         // <
    Greater,      // >
    LessEqual,    // <=
    GreaterEqual, // >=
    And,          // and
    Or,           // or
    Not,          // not

    // Delimiters
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Comma,        // ,
    Colon,        // :
    Semicolon,    // ;

    // Special
    Eof,
    Illegal(String),
}

impl Token {
    pub fn literal(&self) -> String {
        match self {
            Token::Integer(value) => value.to_string(),
            Token::Float(value) => value.to_string(),
            Token::String(value) => value.clone(),
            Token::FString(value) => value.clone(),
            Token::Boolean(value) => value.to_string(),
            Token::Identifier(value) => value.clone(),
            Token::Illegal(value) => value.clone(),
            Token::Comment(value) => value.clone(),
            _ => format!("{self:?}"),
        }
    }
}
