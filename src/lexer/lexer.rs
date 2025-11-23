use crate::lexer::token::Token;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input[self.read_position]
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        // Check for comments
        if self.ch == '#' {
            return self.read_comment();
        }

        // All tokens have already been advanced to the next character
        // except for EOF, so we don't need to do anything here

        match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    self.read_char();
                    Token::Equal
                } else {
                    self.read_char();
                    Token::Assign
                }
            }
            ';' => {
                self.read_char();
                Token::Semicolon
            }
            ':' => {
                self.read_char();
                Token::Colon
            }
            ',' => {
                self.read_char();
                Token::Comma
            }
            '(' => {
                self.read_char();
                Token::LeftParen
            }
            ')' => {
                self.read_char();
                Token::RightParen
            }
            '{' => {
                self.read_char();
                Token::LeftBrace
            }
            '}' => {
                self.read_char();
                Token::RightBrace
            }
            '+' => {
                self.read_char();
                Token::Plus
            }
            '-' => {
                self.read_char();
                Token::Minus
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    self.read_char();
                    Token::NotEqual
                } else {
                    self.read_char();
                    Token::Not
                }
            }
            '/' => {
                if self.peek_char() == '/' {
                    self.read_char();
                    self.read_char();
                    Token::FloorDivide
                } else {
                    self.read_char();
                    Token::Divide
                }
            }
            '%' => {
                self.read_char();
                Token::Modulo
            }
            '*' => {
                if self.peek_char() == '*' {
                    self.read_char();
                    self.read_char();
                    Token::Power
                } else {
                    self.read_char();
                    Token::Multiply
                }
            }
            '<' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    self.read_char();
                    Token::LessEqual
                } else {
                    self.read_char();
                    Token::Less
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    self.read_char();
                    Token::GreaterEqual
                } else {
                    self.read_char();
                    Token::Greater
                }
            }
            '"' => {
                self.read_char(); // skip opening quote
                Token::String(self.read_string())
            }
            '\'' => {
                self.read_char(); // skip opening quote
                Token::String(self.read_string_single())
            }
            '0'..='9' => self.read_number(),
            'a'..='z' | 'A'..='Z' | '_' => {
                // Check if this could be an f-string
                if self.ch == 'f' && (self.peek_char() == '"' || self.peek_char() == '\'') {
                    self.read_char(); // consume 'f'
                    if self.ch == '"' {
                        self.read_char(); // skip opening quote
                        Token::FString(self.read_fstring())
                    } else if self.ch == '\'' {
                        self.read_char(); // skip opening quote
                        Token::FString(self.read_fstring_single())
                    } else {
                        // This shouldn't happen, but fallback to identifier
                        let ident = self.read_identifier();
                        Token::Identifier(ident)
                    }
                } else {
                    let ident = self.read_identifier();
                    match ident.as_str() {
                        "def" => Token::Def,
                        "if" => Token::If,
                        "else" => Token::Else,
                        "while" => Token::While,
                        "return" => Token::Return,
                        "True" => Token::Boolean(true),
                        "False" => Token::Boolean(false),
                        "None" => Token::None,
                        "and" => Token::And,
                        "or" => Token::Or,
                        "not" => Token::Not,
                        _ => Token::Identifier(ident),
                    }
                }
            }
            '\0' => Token::Eof,
            _ => {
                let ch = self.ch;
                self.read_char();
                Token::Illegal(ch.to_string())
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        self.input[start..self.position].iter().collect()
    }

    fn read_number(&mut self) -> Token {
        let start = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }

        if self.ch == '.' && is_digit(self.peek_char()) {
            self.read_char(); // consume the dot
            while is_digit(self.ch) {
                self.read_char();
            }
            let float_str: String = self.input[start..self.position].iter().collect();
            Token::Float(float_str.parse().unwrap_or(0.0))
        } else {
            let int_str: String = self.input[start..self.position].iter().collect();
            Token::Integer(int_str.parse().unwrap_or(0))
        }
    }

    fn read_string(&mut self) -> String {
        let mut result = String::new();
        while self.ch != '"' && self.ch != '\0' {
            if self.ch == '\\' {
                self.read_char(); // consume the backslash
                match self.ch {
                    'n' => result.push('\n'),
                    't' => result.push('\t'),
                    'r' => result.push('\r'),
                    '"' => result.push('"'),
                    '\'' => result.push('\''),
                    '\\' => result.push('\\'),
                    _ => {
                        // If it's not a recognized escape sequence,
                        // just add the backslash and the character as-is
                        result.push('\\');
                        result.push(self.ch);
                    }
                }
            } else {
                result.push(self.ch);
            }
            self.read_char();
        }
        if self.ch == '"' {
            self.read_char(); // consume closing quote
        }
        result
    }

    fn read_string_single(&mut self) -> String {
        let mut result = String::new();
        while self.ch != '\'' && self.ch != '\0' {
            if self.ch == '\\' {
                self.read_char(); // consume the backslash
                match self.ch {
                    'n' => result.push('\n'),
                    't' => result.push('\t'),
                    'r' => result.push('\r'),
                    '"' => result.push('"'),
                    '\'' => result.push('\''),
                    '\\' => result.push('\\'),
                    _ => {
                        // If it's not a recognized escape sequence,
                        // just add the backslash and the character as-is
                        result.push('\\');
                        result.push(self.ch);
                    }
                }
            } else {
                result.push(self.ch);
            }
            self.read_char();
        }
        if self.ch == '\'' {
            self.read_char(); // consume closing quote
        }
        result
    }

    fn read_comment(&mut self) -> Token {
        let start = self.position;
        // Skip the '#' character
        self.read_char();
        // Read until end of line or end of file
        while self.ch != '\n' && self.ch != '\0' {
            self.read_char();
        }
        let comment_text: String = self.input[start + 1..self.position].iter().collect();
        Token::Comment(comment_text)
    }

    fn read_fstring(&mut self) -> String {
        let mut result = String::new();
        let mut brace_depth = 0;
        let mut in_expression = false;

        while self.ch != '"' && self.ch != '\0' {
            if self.ch == '\\' {
                // Handle escape sequences
                self.read_char(); // consume the backslash
                match self.ch {
                    'n' => result.push('\n'),
                    't' => result.push('\t'),
                    'r' => result.push('\r'),
                    '"' => result.push('"'),
                    '\'' => result.push('\''),
                    '\\' => result.push('\\'),
                    '{' => result.push('{'), // Escaped brace
                    '}' => result.push('}'), // Escaped brace
                    _ => {
                        // If it's not a recognized escape sequence,
                        // just add the backslash and the character as-is
                        result.push('\\');
                        result.push(self.ch);
                    }
                }
            } else if self.ch == '{' {
                if in_expression {
                    brace_depth += 1;
                }
                in_expression = true;
                result.push(self.ch);
            } else if self.ch == '}' {
                if in_expression {
                    if brace_depth > 0 {
                        brace_depth -= 1;
                    } else {
                        in_expression = false;
                    }
                }
                result.push(self.ch);
            } else {
                result.push(self.ch);
            }
            self.read_char();
        }

        if self.ch == '"' {
            self.read_char(); // consume closing quote
        }
        result
    }

    fn read_fstring_single(&mut self) -> String {
        let mut result = String::new();
        let mut brace_depth = 0;
        let mut in_expression = false;

        while self.ch != '\'' && self.ch != '\0' {
            if self.ch == '\\' {
                // Handle escape sequences
                self.read_char(); // consume the backslash
                match self.ch {
                    'n' => result.push('\n'),
                    't' => result.push('\t'),
                    'r' => result.push('\r'),
                    '"' => result.push('"'),
                    '\'' => result.push('\''),
                    '\\' => result.push('\\'),
                    '{' => result.push('{'), // Escaped brace
                    '}' => result.push('}'), // Escaped brace
                    _ => {
                        // If it's not a recognized escape sequence,
                        // just add the backslash and the character as-is
                        result.push('\\');
                        result.push(self.ch);
                    }
                }
            } else if self.ch == '{' {
                if in_expression {
                    brace_depth += 1;
                }
                in_expression = true;
                result.push(self.ch);
            } else if self.ch == '}' {
                if in_expression {
                    if brace_depth > 0 {
                        brace_depth -= 1;
                    } else {
                        in_expression = false;
                    }
                }
                result.push(self.ch);
            } else {
                result.push(self.ch);
            }
            self.read_char();
        }

        if self.ch == '\'' {
            self.read_char(); // consume closing quote
        }
        result
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_' || ch.is_numeric()
}

fn is_digit(ch: char) -> bool {
    ch.is_numeric()
}
