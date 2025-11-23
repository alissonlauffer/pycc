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
    FString(String), // F-string literal
    Boolean(bool),
    None,
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
