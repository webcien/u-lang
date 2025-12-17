// type_checker.rs — U v0.6
// MIT License — Copyright (c) 2025 Webcien and U contributors

use crate::parser::{Declaration, Expression, Function, Literal, Statement, Type};
use std::collections::HashMap;

use std::fmt;

#[derive(Debug)]
pub enum TypeError {
    Mismatch {
        expected: String,
        actual: String,
        location: String,
    },
    UnknownVariable(String, String),
    InvalidOwnership(String),
    NullNotAllowed(String),
}

pub type Result<T> = std::result::Result<T, TypeError>;

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TypeError::Mismatch { expected, actual, location } => {
                write!(f, "Type mismatch at {}: expected {}, got {}", location, expected, actual)
            }
            TypeError::UnknownVariable(name, context) => {
                write!(f, "Unknown variable {} {}", name, context)
            }
            TypeError::InvalidOwnership(msg) => write!(f, "Ownership error: {}", msg),
            TypeError::NullNotAllowed(msg) => write!(f, "Null not allowed: {}", msg),
        }
    }
}

impl std::error::Error for TypeError {}

// Lexical environment: maps identifiers to their type + ownership state
#[derive(Clone, Debug)]
pub struct Symbol {
    pub ty: Type,
    pub mutable: bool,
    pub moved: bool, // marks if the value was transferred (no longer usable)
}

pub struct TypeChecker {
    // Stack of environments (for functions, blocks, etc.)
    scopes: Vec<HashMap<String, Symbol>>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()], // global scope
        }
    }

    fn current_scope(&mut self) -> &mut HashMap<String, Symbol> {
        self.scopes.last_mut().unwrap()
    }

    fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn check_program(&mut self, declarations: Vec<Declaration>) -> Result<()> {
        // First: register all functions (for forward resolution)
        for decl in &declarations {
            match decl {
                Declaration::Function(f) => {
                    // We only register the signature; the body is verified later
                    // In MVP, there is no overloading, so the name is unique
                }
                Declaration::Actor(_) => {
                    // Actor parsing not implemented in MVP → omitted
                }
            }
        }

        // Then: verify bodies
        for decl in declarations {
            self.check_declaration(decl)?;
        }
        Ok(())
    }

    fn check_declaration(&mut self, decl: Declaration) -> Result<()> {
        match decl {
            Declaration::Function(f) => self.check_function(f)?,
            Declaration::Actor(_) => {
                // MVP: actors not implemented in parser → not verified yet
            }
        }
        Ok(())
    }

    fn check_function(&mut self, f: Function) -> Result<()> {
        self.enter_scope();

        // Register parameters
        for (name, ty) in f.params {
            self.current_scope().insert(
                name.clone(),
                Symbol {
                    ty: ty.clone(),
                    mutable: false, // parameters are immutable by default
                    moved: false,
                },
            );
        }

        // Verify body
        for stmt in f.body {
            self.check_statement(stmt)?;
        }

        self.exit_scope();
        Ok(())
    }

    fn check_statement(&mut self, stmt: Statement) -> Result<()> {
        match stmt {
            Statement::Let { name, mutable, value } => {
                let value_ty = self.check_expression(value)?;
                // In U, all types are non-nullable → OK by construction
                self.current_scope().insert(
                    name,
                    Symbol {
                        ty: value_ty,
                        mutable,
                        moved: false,
                    },
                );
            }
            Statement::Expr(expr) => {
                self.check_expression(expr)?;
            }
            Statement::Return(expr) => {
                self.check_expression(expr)?;
                // Return type validation vs signature → omitted in simple MVP
            }
            Statement::If { condition, then_branch, else_branch } => {
                let _cond_ty = self.check_expression(condition)?;
                for stmt in then_branch {
                    self.check_statement(stmt)?;
                }
                if let Some(else_stmts) = else_branch {
                    for stmt in else_stmts {
                        self.check_statement(stmt)?;
                    }
                }
            }
            Statement::While { condition, body } => {
                let _cond_ty = self.check_expression(condition)?;
                for stmt in body {
                    self.check_statement(stmt)?;
                }
            }
            Statement::For { variable, iterable, body } => {
                let _iter_ty = self.check_expression(iterable)?;
                self.current_scope().insert(
                    variable,
                    Symbol {
                        ty: Type::I32,
                        mutable: false,
                        moved: false,
                    },
                );
                for stmt in body {
                    self.check_statement(stmt)?;
                }
            }
            Statement::Break | Statement::Continue => {
                // Valid in loop context; full validation in v0.8
            }
        }
        Ok(())
    }

    fn check_expression(&mut self, expr: Expression) -> Result<Type> {
        match expr {
            Expression::Literal(lit) => match lit {
                Literal::Integer(_) => Ok(Type::I32),
                Literal::String(_) => Ok(Type::Str),
                Literal::Boolean(_) => Ok(Type::Bool),
            },
            Expression::Identifier(name) => {
                // Search in scopes (from top to bottom)
                for scope in self.scopes.iter().rev() {
                    if let Some(symbol) = scope.get(&name) {
                        if symbol.moved {
                            return Err(TypeError::InvalidOwnership(format!(
                                "Cannot use moved value '{}'", name
                            )));
                        }
                        return Ok(symbol.ty.clone());
                    }
                }
                Err(TypeError::UnknownVariable(
                    name,
                    "in current scope".to_string(),
                ))
            }
            Expression::FunctionCall { name: _, arguments } => {
                // In MVP, we assume that all called functions exist
                // and return `()` (void). To improve in v0.7.
                for arg in arguments {
                    self.check_expression(arg)?;
                }
                // For simplicity in MVP: all external functions return ()
                Ok(Type::I32) // placeholder; in MVP we don't use return value of external functions
            }
            Expression::MethodCall {
                receiver,
                method: _,
                arguments,
            } => {
                // Verify that the receiver exists and has not been moved
                let _recv_ty = self.check_expression(Expression::Identifier(receiver))?;
                // In MVP, actors are not implemented → we simulate with calls
                for arg in arguments {
                    self.check_expression(arg)?;
                }
                // The `.await` in the example is ignored in the current AST → will be handled in v0.7
                Ok(Type::I32) // placeholder
            }
            Expression::Binary { left, operator: _, right } => {
                let _left_ty = self.check_expression(*left)?;
                let _right_ty = self.check_expression(*right)?;
                Ok(Type::I32)
            }
            Expression::Unary { operator: _, operand } => {
                let _op_ty = self.check_expression(*operand)?;
                Ok(Type::I32)
            }
            Expression::Assignment { target, value } => {
                let mut found = false;
                for scope in self.scopes.iter().rev() {
                    if let Some(symbol) = scope.get(&target) {
                        if !symbol.mutable {
                            return Err(TypeError::InvalidOwnership(
                                format!("Cannot assign to immutable variable", )
                            ));
                        }
                        found = true;
                        break;
                    }
                }
                if !found {
                    return Err(TypeError::UnknownVariable(
                        target.clone(),
                        "in current scope".to_string(),
                    ));
                }
                let _value_ty = self.check_expression(*value)?;
                Ok(Type::I32)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lexer::Lexer, parser::Parser};

    #[test]
    fn test_simple_ownership() {
        let source = "fn main() { let x = 42; let y = x; }".to_string();
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let decls = parser.parse().unwrap();
        let mut checker = TypeChecker::new();
        let result = checker.check_program(decls);
        // In MVP, we don't mark 'x' as moved yet → this will be refined in v0.7
        assert!(result.is_ok());
    }
}
