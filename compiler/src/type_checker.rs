// type_checker.rs — U v0.8
// MIT License — Copyright (c) 2025 Webcien and U contributors

use crate::parser::{Declaration, Expression, Function, Literal, Statement, Type, Actor, Trait, TypeDef, TraitImpl};
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

#[derive(Clone, Debug)]
pub struct Symbol {
    pub ty: Type,
    pub mutable: bool,
    pub moved: bool,
}

pub struct TypeChecker {
    scopes: Vec<HashMap<String, Symbol>>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
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
        for decl in &declarations {
            match decl {
                Declaration::Function(_) => {
                    // Function signature registration
                }
                Declaration::Actor(_) => {
                    // Actor registration deferred to v0.9
                }
                Declaration::Trait(_) => {
                    // Trait registration deferred to v0.9
                }
                Declaration::TypeDef(_) => {
                    // Type definition registration deferred to v0.9
                }
                Declaration::TraitImpl(_) => {
                    // Trait implementation registration deferred to v0.9
                }
                Declaration::ExternBlock(_) => {
                    // FFI: extern function registration
                }
            }
        }

        for decl in declarations {
            self.check_declaration(decl)?;
        }
        Ok(())
    }

    fn check_declaration(&mut self, decl: Declaration) -> Result<()> {
        match decl {
            Declaration::Function(f) => self.check_function(f)?,
            Declaration::Actor(_) => {
                // Actors: message passing verification deferred to v0.9
            }
            Declaration::Trait(_) => {
                // Traits: method signature verification deferred to v0.9
            }
            Declaration::TypeDef(_) => {
                // Type definitions: field verification deferred to v0.9
            }
            Declaration::TraitImpl(_) => {
                // Trait implementations: method verification deferred to v0.9
            }
            Declaration::ExternBlock(_) => {
                // FFI: extern blocks are validated at parse time
                // Type checking for FFI calls happens in unsafe blocks
            }
        }
        Ok(())
    }

    fn check_function(&mut self, f: Function) -> Result<()> {
        self.enter_scope();

        for (param_name, param_type) in f.params {
            self.current_scope().insert(
                param_name,
                Symbol {
                    ty: param_type,
                    mutable: false,
                    moved: false,
                },
            );
        }

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
                // Valid in loop context
            }
            Statement::Unsafe { body } => {
                // Unsafe blocks: FFI calls and other unsafe operations
                // Type checking is relaxed but still performed
                for stmt in body {
                    self.check_statement(stmt)?;
                }
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
                for arg in arguments {
                    self.check_expression(arg)?;
                }
                Ok(Type::I32)
            }
            Expression::MethodCall {
                receiver,
                method: _,
                arguments,
            } => {
                let _recv_ty = self.check_expression(Expression::Identifier(receiver))?;
                for arg in arguments {
                    self.check_expression(arg)?;
                }
                Ok(Type::I32)
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
                                "Cannot assign to immutable variable".to_string()
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

    fn type_to_string(&self, ty: &Type) -> String {
        match ty {
            Type::I32 => "i32".to_string(),
            Type::Str => "str".to_string(),
            Type::Bool => "bool".to_string(),
            Type::Ptr => "ptr".to_string(),
            Type::Option(inner) => format!("Option<{}>", self.type_to_string(inner)),
            Type::Result(ok, err) => format!("Result<{}, {}>", self.type_to_string(ok), self.type_to_string(err)),
            Type::Custom(name) => name.clone(),
            Type::Generic { name, type_args } => {
                let args = type_args.iter()
                    .map(|t| self.type_to_string(t))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}<{}>", name, args)
            }
            Type::FunctionPointer { params, return_type } => {
                let param_str = params.iter()
                    .map(|t| self.type_to_string(t))
                    .collect::<Vec<_>>()
                    .join(", ");
                if let Some(ret) = return_type {
                    format!("fn({}) -> {}", param_str, self.type_to_string(ret))
                } else {
                    format!("fn({})", param_str)
                }
            }
        }
    }
}

#[allow(dead_code)]
fn _check_actor(_actor: Actor) {
    // Actor verification deferred to v0.9
}

#[allow(dead_code)]
fn _check_trait(_trait: Trait) {
    // Trait verification deferred to v0.9
}

#[allow(dead_code)]
fn _check_type_def(_typedef: TypeDef) {
    // Type definition verification deferred to v0.9
}

#[allow(dead_code)]
fn _check_trait_impl(_impl: TraitImpl) {
    // Trait implementation verification deferred to v0.9
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lexer::Lexer, parser::Parser};

    #[test]
    fn test_simple_ownership() {
        let source = "fn main() { let x = 42; }".to_string();
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let decls = parser.parse().unwrap();
        let mut checker = TypeChecker::new();
        assert!(checker.check_program(decls).is_ok());
    }
}
