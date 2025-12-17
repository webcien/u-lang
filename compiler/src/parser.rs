// parser.rs — U v0.6
// MIT License — Copyright (c) 2025 Webcien and U contributors

use crate::lexer::{Token, TokenType};

#[derive(Debug, Clone)]
pub enum Type {
    I32,
    Str,
    Bool,
    Option(Box<Type>),
    Result(Box<Type>, Box<Type>),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    FunctionCall {
        name: String,
        arguments: Vec<Expression>,
    },
    MethodCall {
        receiver: String,
        method: String,
        arguments: Vec<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: BinaryOp,
        right: Box<Expression>,
    },
    Unary {
        operator: UnaryOp,
        operand: Box<Expression>,
    },
    Assignment {
        target: String,
        value: Box<Expression>,
    },
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Not,
    Negate,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Integer(u32),
    String(String),
    Boolean(bool),
}

#[derive(Debug, Clone)]
pub enum Statement {
    Let {
        name: String,
        mutable: bool, // false = let, true = var
        value: Expression,
    },
    Expr(Expression),
    Return(Expression),
    If {
        condition: Expression,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    For {
        variable: String,
        iterable: Expression,
        body: Vec<Statement>,
    },
    Break,
    Continue,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<(String, Type)>,
    pub return_type: Option<Type>,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct Actor {
    pub name: String,
    pub methods: Vec<Function>,
}

#[derive(Debug, Clone)]
pub enum Declaration {
    Function(Function),
    Actor(Actor),
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Declaration>, String> {
        let mut declarations = Vec::new();
        while !self.is_at_end() {
            declarations.push(self.parse_declaration()?);
        }
        Ok(declarations)
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn check(&self, kind: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            match (&self.peek().kind, &kind) {
                (TokenType::Identifier(_), TokenType::Identifier(_)) => true,
                (TokenType::StringLiteral(_), TokenType::StringLiteral(_)) => true,
                (TokenType::IntegerLiteral(_), TokenType::IntegerLiteral(_)) => true,
                (a, b) => a == b,
            }
        }
    }

    fn match_token(&mut self, kind: TokenType) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn consume(&mut self, kind: TokenType, message: &str) -> Result<&Token, String> {
        if self.check(kind) {
            Ok(self.advance())
        } else {
            Err(format!("{} at line {}", message, self.peek().line))
        }
    }

    fn parse_declaration(&mut self) -> Result<Declaration, String> {
        if self.match_token(TokenType::Fn) {
            Ok(Declaration::Function(self.parse_function()?))
        } else if self.match_token(TokenType::Actor) {
            Ok(Declaration::Actor(self.parse_actor()?))
        } else {
            Err(format!(
                "Expected 'fn' or 'actor', got {:?} at line {}",
                self.peek().kind,
                self.peek().line
            ))
        }
    }

    fn parse_function(&mut self) -> Result<Function, String> {
        if !self.check(TokenType::Identifier(String::new())) {
            return Err(format!("Expected function name, got {:?}", self.peek().kind));
        }
        let name_token = self.advance();
        let name = match &name_token.kind {
            TokenType::Identifier(s) => s.clone(),
            _ => return Err("Invalid function name".to_string()),
        };

        self.consume(TokenType::LeftParen, "Expected '(' after function name")?;
        let mut params = Vec::new();
        if !self.check(TokenType::RightParen) {
            params.push(self.parse_param()?);
            while self.match_token(TokenType::Comma) {
                params.push(self.parse_param()?);
            }
        }
        self.consume(TokenType::RightParen, "Expected ')' after parameters")?;

        let return_type = if self.match_token(TokenType::Arrow) {
            Some(self.parse_type()?)
        } else {
            None
        };

        let body = self.parse_block()?;
        Ok(Function {
            name,
            params,
            return_type,
            body,
        })
    }

    fn parse_actor(&mut self) -> Result<Actor, String> {
        let name_token = self.consume(TokenType::Identifier(String::new()), "Expected actor name")?;
        let name = match &name_token.kind {
            TokenType::Identifier(s) => s.clone(),
            _ => return Err("Invalid actor name".to_string()),
        };

        let body = self.parse_block()?;
        // In MVP, we assume that all declarations in the block are methods
        let mut methods = Vec::new();
        for _stmt in body {
            // Note: This is simplified. In practice, we would need to distinguish
            // between method declaration and statement within the actor.
            // For MVP, we assume only methods (without complex initial state).
            return Err("Actor parsing not fully implemented in MVP parser".to_string());
        }

        Ok(Actor { name, methods })
    }

    fn parse_param(&mut self) -> Result<(String, Type), String> {
        let name_token = self.consume(TokenType::Identifier(String::new()), "Expected parameter name")?;
        let name = match &name_token.kind {
            TokenType::Identifier(s) => s.clone(),
            _ => return Err("Invalid parameter name".to_string()),
        };

        self.consume(TokenType::Colon, "Expected ':' after parameter name")?;
        let param_type = self.parse_type()?;
        Ok((name, param_type))
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        if self.match_token(TokenType::I32) {
            Ok(Type::I32)
        } else if self.match_token(TokenType::Str) {
            Ok(Type::Str)
        } else if self.match_token(TokenType::Bool) {
            Ok(Type::Bool)
        } else if self.match_token(TokenType::Option) {
            self.consume(TokenType::LessThan, "Expected '<' after Option")?;
            let inner = self.parse_type()?;
            self.consume(TokenType::GreaterThan, "Expected '>' after Option type")?;
            Ok(Type::Option(Box::new(inner)))
        } else if self.match_token(TokenType::Result) {
            self.consume(TokenType::LessThan, "Expected '<' after Result")?;
            let ok_type = self.parse_type()?;
            self.consume(TokenType::Comma, "Expected ',' in Result type")?;
            let err_type = self.parse_type()?;
            self.consume(TokenType::GreaterThan, "Expected '>' after Result types")?;
            Ok(Type::Result(Box::new(ok_type), Box::new(err_type)))
        } else {
            Err(format!(
                "Expected type, got {:?} at line {}",
                self.peek().kind,
                self.peek().line
            ))
        }
    }

    fn parse_block(&mut self) -> Result<Vec<Statement>, String> {
        self.consume(TokenType::LeftBrace, "Expected '{'")?;
        let mut statements = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }
        self.consume(TokenType::RightBrace, "Expected '}'")?;
        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        if self.match_token(TokenType::If) {
            let condition = self.parse_expression()?;
            let then_branch = self.parse_block()?;
            let else_branch = if self.match_token(TokenType::Else) {
                Some(self.parse_block()?)
            } else {
                None
            };
            Ok(Statement::If {
                condition,
                then_branch,
                else_branch,
            })
        } else if self.match_token(TokenType::While) {
            let condition = self.parse_expression()?;
            let body = self.parse_block()?;
            Ok(Statement::While { condition, body })
        } else if self.match_token(TokenType::For) {
            let var_token = self.consume(TokenType::Identifier(String::new()), "Expected variable name in for loop")?;
            let variable = match &var_token.kind {
                TokenType::Identifier(s) => s.clone(),
                _ => return Err("Invalid variable name".to_string()),
            };
            self.consume(TokenType::In, "Expected 'in' in for loop")?;
            let iterable = self.parse_expression()?;
            let body = self.parse_block()?;
            Ok(Statement::For { variable, iterable, body })
        } else if self.match_token(TokenType::Break) {
            self.consume(TokenType::Semicolon, "Expected ';' after break")?;
            Ok(Statement::Break)
        } else if self.match_token(TokenType::Continue) {
            self.consume(TokenType::Semicolon, "Expected ';' after continue")?;
            Ok(Statement::Continue)
        } else if self.match_token(TokenType::Let) {
            let name_token = self.consume(TokenType::Identifier(String::new()), "Expected variable name after 'let'")?;
            let name = match &name_token.kind {
                TokenType::Identifier(s) => s.clone(),
                _ => return Err("Invalid variable name".to_string()),
            };
            self.consume(TokenType::Equal, "Expected '=' after variable name")?;
            let value = self.parse_expression()?;
            self.consume(TokenType::Semicolon, "Expected ';' after let statement")?;
            Ok(Statement::Let {
                name,
                mutable: false,
                value,
            })
        } else if self.match_token(TokenType::Var) {
            let name_token = self.consume(TokenType::Identifier(String::new()), "Expected variable name after 'var'")?;
            let name = match &name_token.kind {
                TokenType::Identifier(s) => s.clone(),
                _ => return Err("Invalid variable name".to_string()),
            };
            self.consume(TokenType::Equal, "Expected '=' after variable name")?;
            let value = self.parse_expression()?;
            self.consume(TokenType::Semicolon, "Expected ';' after var statement")?;
            Ok(Statement::Let {
                name,
                mutable: true,
                value,
            })
        } else if self.match_token(TokenType::Return) {
            let expr = self.parse_expression()?;
            self.consume(TokenType::Semicolon, "Expected ';' after return")?;
            Ok(Statement::Return(expr))
        } else {
            let expr = self.parse_expression()?;
            self.consume(TokenType::Semicolon, "Expected ';' after expression")?;
            Ok(Statement::Expr(expr))
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Expression, String> {
        let expr = self.parse_or()?;
        
        // Check if this is an assignment
        if self.match_token(TokenType::Equal) {
            if let Expression::Identifier(target) = expr {
                let value = self.parse_assignment()?;
                return Ok(Expression::Assignment {
                    target,
                    value: Box::new(value),
                });
            } else {
                return Err("Invalid assignment target".to_string());
            }
        }
        
        Ok(expr)
    }

    fn parse_or(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_and()?;
        while self.match_token(TokenType::Or) {
            let right = self.parse_and()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: BinaryOp::Or,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_and(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_equality()?;
        while self.match_token(TokenType::And) {
            let right = self.parse_equality()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: BinaryOp::And,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_comparison()?;
        while let Some(op) = self.match_equality_op() {
            let right = self.parse_comparison()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn match_equality_op(&mut self) -> Option<BinaryOp> {
        if self.match_token(TokenType::DoubleEqual) {
            Some(BinaryOp::Equal)
        } else if self.match_token(TokenType::NotEqual) {
            Some(BinaryOp::NotEqual)
        } else {
            None
        }
    }

    fn parse_comparison(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_addition()?;
        while let Some(op) = self.match_comparison_op() {
            let right = self.parse_addition()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn match_comparison_op(&mut self) -> Option<BinaryOp> {
        if self.match_token(TokenType::LessThan) {
            Some(BinaryOp::Less)
        } else if self.match_token(TokenType::LessEqual) {
            Some(BinaryOp::LessEqual)
        } else if self.match_token(TokenType::GreaterThan) {
            Some(BinaryOp::Greater)
        } else if self.match_token(TokenType::GreaterEqual) {
            Some(BinaryOp::GreaterEqual)
        } else {
            None
        }
    }

    fn parse_addition(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_multiplication()?;
        while let Some(op) = self.match_addition_op() {
            let right = self.parse_multiplication()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn match_addition_op(&mut self) -> Option<BinaryOp> {
        if self.match_token(TokenType::Plus) {
            Some(BinaryOp::Add)
        } else if self.match_token(TokenType::Minus) {
            Some(BinaryOp::Subtract)
        } else {
            None
        }
    }

    fn parse_multiplication(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_unary()?;
        while let Some(op) = self.match_multiplication_op() {
            let right = self.parse_unary()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn match_multiplication_op(&mut self) -> Option<BinaryOp> {
        if self.match_token(TokenType::Star) {
            Some(BinaryOp::Multiply)
        } else if self.match_token(TokenType::Slash) {
            Some(BinaryOp::Divide)
        } else if self.match_token(TokenType::Percent) {
            Some(BinaryOp::Modulo)
        } else {
            None
        }
    }

    fn parse_unary(&mut self) -> Result<Expression, String> {
        if self.match_token(TokenType::Not) {
            let operand = self.parse_unary()?;
            Ok(Expression::Unary {
                operator: UnaryOp::Not,
                operand: Box::new(operand),
            })
        } else if self.match_token(TokenType::Minus) {
            let operand = self.parse_unary()?;
            Ok(Expression::Unary {
                operator: UnaryOp::Negate,
                operand: Box::new(operand),
            })
        } else {
            self.parse_call()
        }
    }

    fn parse_call(&mut self) -> Result<Expression, String> {
        // In MVP: we support only calls, literals and identifiers
        if self.check(TokenType::StringLiteral(String::new())) {
            let token = self.advance();
            let value = match &token.kind {
                TokenType::StringLiteral(s) => s.clone(),
                _ => unreachable!(),
            };
            Ok(Expression::Literal(Literal::String(value)))
        } else if self.check(TokenType::IntegerLiteral(0)) {
            let token = self.advance();
            let value = match &token.kind {
                TokenType::IntegerLiteral(n) => *n,
                _ => unreachable!(),
            };
            Ok(Expression::Literal(Literal::Integer(value)))
        } else if self.check(TokenType::Identifier(String::new())) {
            let token = self.advance();
            let ident = match &token.kind {
                TokenType::Identifier(s) => s.clone(),
                _ => unreachable!(),
            };

            if self.check(TokenType::LeftParen) {
                // Function call
                self.advance(); // consume '('
                let mut args = Vec::new();
                if !self.check(TokenType::RightParen) {
                    args.push(self.parse_expression()?);
                    while self.match_token(TokenType::Comma) {
                        args.push(self.parse_expression()?);
                    }
                }
                self.consume(TokenType::RightParen, "Expected ')' after function arguments")?;
                Ok(Expression::FunctionCall {
                    name: ident,
                    arguments: args,
                })
            } else if self.check(TokenType::Dot) {
                // Method call: receiver.method(...)
                self.advance(); // consume '.'
                let method_token = self.consume(TokenType::Identifier(String::new()), "Expected method name")?;
                let method = match &method_token.kind {
                    TokenType::Identifier(s) => s.clone(),
                    _ => return Err("Invalid method name".to_string()),
                };
                self.consume(TokenType::LeftParen, "Expected '(' after method name")?;
                let mut args = Vec::new();
                if !self.check(TokenType::RightParen) {
                    args.push(self.parse_expression()?);
                    while self.match_token(TokenType::Comma) {
                        args.push(self.parse_expression()?);
                    }
                }
                self.consume(TokenType::RightParen, "Expected ')' after method arguments")?;
                Ok(Expression::MethodCall {
                    receiver: ident,
                    method,
                    arguments: args,
                })
            } else {
                Ok(Expression::Identifier(ident))
            }
        } else {
            Err(format!(
                "Unexpected token in expression: {:?} at line {}",
                self.peek().kind,
                self.peek().line
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_hello_function() {
        let source = "fn main() { print(\"Hello\"); }".to_string();
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_ok());
    }

    #[test]
    fn test_while_loop() {
        let source = "fn main() { var i = 0; while i < 10 { i = i + 1; } }".to_string();
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_ok());
    }
}
