#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    // Program node containing all statements
    Program(Program),

    // Statement nodes
    Function(Function),
    Assignment(Assignment),
    #[allow(dead_code)]
    If(If),
    #[allow(dead_code)]
    While(While),
    Return(Return),
    ExpressionStatement(Expression),

    // Expression nodes
    Binary(Binary),
    Unary(Unary),
    Literal(Literal),
    Identifier(Identifier),
    Call(Call),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub name: String,
    pub value: Box<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct If {
    pub condition: Box<Node>,
    pub then_branch: Box<Node>,
    pub else_branch: Option<Box<Node>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct While {
    pub condition: Box<Node>,
    pub body: Box<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Return {
    pub value: Option<Box<Node>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub expression: Box<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Binary {
    pub left: Box<Node>,
    pub operator: BinaryOperator,
    pub right: Box<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    FloorDivide,
    Modulo,
    Power,
    #[allow(dead_code)]
    Equal,
    #[allow(dead_code)]
    NotEqual,
    #[allow(dead_code)]
    Less,
    #[allow(dead_code)]
    Greater,
    #[allow(dead_code)]
    LessEqual,
    #[allow(dead_code)]
    GreaterEqual,
    #[allow(dead_code)]
    And,
    #[allow(dead_code)]
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Unary {
    pub operator: UnaryOperator,
    pub operand: Box<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Plus,
    Minus,
    #[allow(dead_code)]
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Literal {
    pub value: LiteralValue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Integer(i64),
    Float(f64),
    String(String),
    FString(FString), // F-string with parsed expressions
    Boolean(bool),
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FString {
    pub parts: Vec<FStringPart>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FStringPart {
    Literal(String),
    Expression(String), // For now, store as string - will be parsed later
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: Box<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Call {
    pub callee: String,
    pub arguments: Vec<Node>,
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

impl Program {
    pub fn new() -> Self {
        Program {
            statements: Vec::new(),
        }
    }
}

impl FString {
    pub fn parse(content: &str) -> Self {
        let mut parts = Vec::new();
        let mut current_literal = String::new();
        let mut current_expression = String::new();
        let mut in_expression = false;
        let mut brace_depth = 0;
        let mut chars = content.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '\\' {
                // Handle escape sequences - add to current part
                if in_expression {
                    current_expression.push(ch);
                } else {
                    current_literal.push(ch);
                }
                // Add the escaped character
                if let Some(&_next_ch) = chars.peek() {
                    let escaped = chars.next().unwrap();
                    if in_expression {
                        current_expression.push(escaped);
                    } else {
                        current_literal.push(escaped);
                    }
                }
            } else if ch == '{' {
                if in_expression {
                    brace_depth += 1;
                    current_expression.push(ch);
                } else {
                    // Start of expression
                    if !current_literal.is_empty() {
                        parts.push(FStringPart::Literal(current_literal.clone()));
                        current_literal.clear();
                    }
                    in_expression = true;
                    current_expression.clear();
                }
            } else if ch == '}' {
                if in_expression {
                    if brace_depth > 0 {
                        brace_depth -= 1;
                        current_expression.push(ch);
                    } else {
                        // End of expression
                        if !current_expression.is_empty() {
                            parts.push(FStringPart::Expression(current_expression.clone()));
                            current_expression.clear();
                        }
                        in_expression = false;
                    }
                } else {
                    current_literal.push(ch);
                }
            } else if in_expression {
                current_expression.push(ch);
            } else {
                current_literal.push(ch);
            }
        }

        // Add any remaining literal part
        if !current_literal.is_empty() {
            parts.push(FStringPart::Literal(current_literal));
        }

        FString { parts }
    }
}
