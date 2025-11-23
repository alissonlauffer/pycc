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
    // True, False are handled as Boolean literals instead
    // True,
    // False,

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
    LeftParen,  // (
    RightParen, // )
    LeftBrace,  // {
    RightBrace, // }
    // LeftBracket,  // [ - Not currently used
    // RightBracket, // ] - Not currently used
    Comma,     // ,
    Colon,     // :
    Semicolon, // ;

    // Special
    Eof,
    Illegal(String),
}
