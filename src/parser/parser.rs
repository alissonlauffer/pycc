use crate::ast::{
    Assignment, Binary, BinaryOperator, Identifier, Literal, LiteralValue, Node, Program,
};
use crate::lexer::{Lexer, Token};

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: Token::Eof,
            peek_token: Token::Eof,
        };
        parser.next_token(); // Initialize current_token
        parser.next_token(); // Initialize peek_token
        parser
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn peek_token(&self) -> &Token {
        &self.peek_token
    }

    pub fn parse_program(&mut self) -> Node {
        let mut program = Program::new();

        while self.current_token != Token::Eof {
            // Skip comment tokens
            if matches!(self.current_token, Token::Comment(_)) {
                self.next_token();
                continue;
            }

            if let Some(statement) = self.parse_statement() {
                program.statements.push(statement);
            } else {
                // If we couldn't parse a statement, advance to the next token
                // to avoid infinite loops
                self.next_token();
            }
        }

        Node::Program(program)
    }

    fn parse_statement(&mut self) -> Option<Node> {
        match &self.current_token {
            Token::Def => self.parse_function_definition(),
            Token::Identifier(_) => {
                // Could be an assignment or a function call
                self.parse_statement_with_identifier()
            }
            Token::Return => self.parse_return_statement(),
            _ => {
                // For now, treat everything else as an expression statement
                self.parse_expression_statement()
            }
        }
    }

    fn parse_statement_with_identifier(&mut self) -> Option<Node> {
        // Look ahead to see if this is an assignment
        if let Token::Identifier(name) = &self.current_token {
            // Check if the next token is '=' for assignment
            if self.peek_token() == &Token::Assign {
                // This is an assignment
                let name_clone = name.clone();
                self.next_token(); // consume identifier
                self.next_token(); // consume '='
                if let Some(value) = self.parse_expression() {
                    return Some(Node::Assignment(Assignment {
                        name: name_clone,
                        value: Box::new(value),
                    }));
                }
            } else {
                // This is a function call or other expression
                return self.parse_expression_statement();
            }
        }

        None
    }

    fn parse_return_statement(&mut self) -> Option<Node> {
        self.next_token(); // consume 'return'

        // Check if there's a return value
        if self.current_token != Token::Eof && self.current_token != Token::Semicolon {
            if let Some(value) = self.parse_expression() {
                return Some(Node::Return(crate::ast::Return {
                    value: Some(Box::new(value)),
                }));
            }
        }

        Some(Node::Return(crate::ast::Return { value: None }))
    }

    fn parse_function_definition(&mut self) -> Option<Node> {
        self.next_token(); // consume 'def'

        // Parse function name
        let name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return None;
        };

        self.next_token(); // consume function name

        // Parse parameters
        if self.current_token != Token::LeftParen {
            return None;
        }

        self.next_token(); // consume '('

        let mut parameters = Vec::new();

        // Parse parameter list
        if self.current_token != Token::RightParen {
            while let Token::Identifier(param_name) = &self.current_token {
                parameters.push(param_name.clone());
                self.next_token(); // consume parameter name

                if self.current_token == Token::Comma {
                    self.next_token(); // consume ','
                } else {
                    break;
                }
            }
        }

        if self.current_token != Token::RightParen {
            return None;
        }

        self.next_token(); // consume ')'

        if self.current_token != Token::Colon {
            return None;
        }

        self.next_token(); // consume ':'

        // Parse function body
        // For now, we'll just parse the return statement
        // In a full implementation, we'd parse a block of statements
        let body = self.parse_return_statement()?;

        // Create Function node
        Some(Node::Function(crate::ast::Function {
            name,
            parameters,
            body: Box::new(body),
        }))
    }

    fn parse_expression_statement(&mut self) -> Option<Node> {
        self.parse_expression().map(|expression| {
            Node::ExpressionStatement(crate::ast::Expression {
                expression: Box::new(expression),
            })
        })
    }

    fn parse_expression(&mut self) -> Option<Node> {
        self.parse_additive()
    }

    fn parse_additive(&mut self) -> Option<Node> {
        let mut left = self.parse_multiplicative()?;

        while matches!(self.current_token, Token::Plus | Token::Minus) {
            let operator = match self.current_token {
                Token::Plus => BinaryOperator::Add,
                Token::Minus => BinaryOperator::Subtract,
                _ => break,
            };

            self.next_token(); // consume operator
            let right = self.parse_multiplicative()?;

            left = Node::Binary(Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            });
        }

        Some(left)
    }

    fn parse_multiplicative(&mut self) -> Option<Node> {
        let mut left = self.parse_power()?;

        while matches!(
            self.current_token,
            Token::Multiply | Token::Divide | Token::FloorDivide | Token::Modulo
        ) {
            let operator = match self.current_token {
                Token::Multiply => BinaryOperator::Multiply,
                Token::Divide => BinaryOperator::Divide,
                Token::FloorDivide => BinaryOperator::FloorDivide,
                Token::Modulo => BinaryOperator::Modulo,
                _ => break,
            };

            self.next_token(); // consume operator
            let right = self.parse_power()?;

            left = Node::Binary(Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            });
        }

        Some(left)
    }

    fn parse_power(&mut self) -> Option<Node> {
        let mut left = self.parse_unary()?;

        // Right associative for power operator
        if let Token::Power = self.current_token {
            self.next_token(); // consume operator
            let right = self.parse_power()?;

            left = Node::Binary(Binary {
                left: Box::new(left),
                operator: BinaryOperator::Power,
                right: Box::new(right),
            });
        }

        Some(left)
    }

    fn parse_unary(&mut self) -> Option<Node> {
        match self.current_token {
            Token::Plus => {
                self.next_token(); // consume '+'
                let operand = self.parse_unary()?;
                Some(Node::Unary(crate::ast::Unary {
                    operator: crate::ast::UnaryOperator::Plus,
                    operand: Box::new(operand),
                }))
            }
            Token::Minus => {
                self.next_token(); // consume '-'
                let operand = self.parse_unary()?;
                Some(Node::Unary(crate::ast::Unary {
                    operator: crate::ast::UnaryOperator::Minus,
                    operand: Box::new(operand),
                }))
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Option<Node> {
        match &self.current_token {
            Token::Integer(value) => {
                let node = Node::Literal(Literal {
                    value: LiteralValue::Integer(*value),
                });
                self.next_token();
                Some(node)
            }
            Token::Float(value) => {
                let node = Node::Literal(Literal {
                    value: LiteralValue::Float(*value),
                });
                self.next_token();
                Some(node)
            }
            Token::String(value) => {
                let node = Node::Literal(Literal {
                    value: LiteralValue::String(value.clone()),
                });
                self.next_token();
                Some(node)
            }
            Token::FString(value) => {
                let node = Node::Literal(Literal {
                    value: LiteralValue::FString(value.clone()),
                });
                self.next_token();
                Some(node)
            }
            Token::Boolean(value) => {
                let node = Node::Literal(Literal {
                    value: LiteralValue::Boolean(*value),
                });
                self.next_token();
                Some(node)
            }
            Token::None => {
                let node = Node::Literal(Literal {
                    value: LiteralValue::None,
                });
                self.next_token();
                Some(node)
            }
            Token::Identifier(name) => {
                let name_clone = name.clone();
                self.next_token();

                // Check if this is a function call
                if self.current_token == Token::LeftParen {
                    self.parse_function_call(name_clone)
                } else {
                    Some(Node::Identifier(Identifier { name: name_clone }))
                }
            }
            Token::LeftParen => {
                self.next_token(); // consume '('
                let expr = self.parse_expression();
                if self.current_token == Token::RightParen {
                    self.next_token(); // consume ')'
                    expr
                } else {
                    None // Missing closing parenthesis
                }
            }
            _ => None,
        }
    }

    fn parse_function_call(&mut self, name: String) -> Option<Node> {
        self.next_token(); // consume '('

        let mut arguments = Vec::new();

        // Parse arguments
        if self.current_token != Token::RightParen {
            while let Some(arg) = self.parse_expression() {
                arguments.push(arg);

                if self.current_token == Token::Comma {
                    self.next_token(); // consume ','
                } else {
                    break;
                }
            }
        }

        if self.current_token == Token::RightParen {
            self.next_token(); // consume ')'
            Some(Node::Call(crate::ast::Call {
                callee: name,
                arguments,
            }))
        } else {
            None // Missing closing parenthesis
        }
    }
}
