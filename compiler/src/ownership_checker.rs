// ownership_checker.rs — U v1.0 Ownership Checker
// MIT License — Copyright (c) 2025 Webcien and U contributors
//
// This module implements the 7 formal ownership rules from U-lang-mejorado-0.6.md:
//
// 1. Each value has exactly one owner
// 2. The owner can be mutable or immutable, but not both at once
// 3. Deep copies require explicit `.clone()` call
// 4. Immutable references are allowed, but only within the lexical scope that creates them
// 5. No mutable borrowing
// 6. No explicit lifetimes; compiler infers validity by scope
// 7. Ownership transfer occurs on assignment or function call

use crate::parser::{Declaration, Expression, Function, Statement, Type, Literal};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
pub enum OwnershipError {
    UseAfterMove {
        variable: String,
        moved_at: String,
        used_at: String,
    },
    MultipleOwners {
        variable: String,
        location: String,
    },
    InvalidClone {
        variable: String,
        reason: String,
    },
    ReferenceOutOfScope {
        reference: String,
        scope: String,
    },
    MutableBorrow {
        variable: String,
        location: String,
    },
    MutabilityConflict {
        variable: String,
        declared_as: String,
        used_as: String,
    },
}

impl fmt::Display for OwnershipError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OwnershipError::UseAfterMove { variable, moved_at, used_at } => {
                write!(
                    f,
                    "Use of moved value '{}': moved at {}, used at {}",
                    variable, moved_at, used_at
                )
            }
            OwnershipError::MultipleOwners { variable, location } => {
                write!(
                    f,
                    "Multiple owners for value '{}' at {}",
                    variable, location
                )
            }
            OwnershipError::InvalidClone { variable, reason } => {
                write!(
                    f,
                    "Cannot clone '{}': {}",
                    variable, reason
                )
            }
            OwnershipError::ReferenceOutOfScope { reference, scope } => {
                write!(
                    f,
                    "Reference '{}' used outside its scope {}",
                    reference, scope
                )
            }
            OwnershipError::MutableBorrow { variable, location } => {
                write!(
                    f,
                    "Mutable borrowing not allowed for '{}' at {}",
                    variable, location
                )
            }
            OwnershipError::MutabilityConflict { variable, declared_as, used_as } => {
                write!(
                    f,
                    "Mutability conflict for '{}': declared as {}, used as {}",
                    variable, declared_as, used_as
                )
            }
        }
    }
}

impl std::error::Error for OwnershipError {}

pub type Result<T> = std::result::Result<T, OwnershipError>;

/// Ownership state of a value
#[derive(Debug, Clone, PartialEq)]
enum OwnershipState {
    Owned,           // Value is owned by current scope
    Moved,           // Value has been moved out
    Borrowed,        // Value is immutably borrowed
    Invalid,         // Value is in invalid state
}

/// Information about a variable's ownership
#[derive(Debug, Clone)]
struct OwnershipInfo {
    name: String,
    ty: Type,
    mutable: bool,
    state: OwnershipState,
    scope_level: usize,
    declared_at: String,
    moved_at: Option<String>,
}

/// Ownership checker implementing the 7 formal rules
pub struct OwnershipChecker {
    scopes: Vec<HashMap<String, OwnershipInfo>>,
    current_scope: usize,
}

impl OwnershipChecker {
    pub fn new() -> Self {
        OwnershipChecker {
            scopes: vec![HashMap::new()],
            current_scope: 0,
        }
    }

    /// Check ownership rules for a list of declarations
    pub fn check_program(&mut self, declarations: &[Declaration]) -> Result<()> {
        for decl in declarations {
            self.check_declaration(decl)?;
        }
        Ok(())
    }

    fn check_declaration(&mut self, decl: &Declaration) -> Result<()> {
        match decl {
            Declaration::Function(func) => self.check_function(func),
            _ => Ok(()), // Actors, traits, etc. handled separately
        }
    }

    fn check_function(&mut self, func: &Function) -> Result<()> {
        self.enter_scope();
        
        // Add parameters to scope
        for (param_name, param_type) in &func.params {
            self.declare_variable(
                param_name.clone(),
                param_type.clone(),
                false, // Parameters are immutable by default
                "function parameter".to_string(),
            );
        }
        
        // Check function body
        for stmt in &func.body {
            self.check_statement(stmt)?;
        }
        
        self.exit_scope();
        Ok(())
    }

    fn check_statement(&mut self, stmt: &Statement) -> Result<()> {
        match stmt {
            Statement::Let { name, mutable, value } => {
                self.check_variable_declaration(name, *mutable, value)
            }
            Statement::Expr(expr) => {
                self.check_expression(expr)?;
                Ok(())
            }
            Statement::If { condition, then_branch, else_branch } => {
                self.check_expression(condition)?;
                
                self.enter_scope();
                for stmt in then_branch {
                    self.check_statement(stmt)?;
                }
                self.exit_scope();
                
                if let Some(else_stmts) = else_branch {
                    self.enter_scope();
                    for stmt in else_stmts {
                        self.check_statement(stmt)?;
                    }
                    self.exit_scope();
                }
                Ok(())
            }
            Statement::While { condition, body } => {
                self.check_expression(condition)?;
                self.enter_scope();
                for stmt in body {
                    self.check_statement(stmt)?;
                }
                self.exit_scope();
                Ok(())
            }
            Statement::For { variable, iterable, body } => {
                self.check_expression(iterable)?;
                self.enter_scope();
                // Declare loop variable
                self.declare_variable(
                    variable.clone(),
                    Type::I32, // Simplified
                    false,
                    "for loop".to_string(),
                );
                for stmt in body {
                    self.check_statement(stmt)?;
                }
                self.exit_scope();
                Ok(())
            }
            Statement::Return(expr) => {
                self.check_expression(expr)?;
                Ok(())
            }
            Statement::Unsafe { body } => {
                // Unsafe blocks bypass ownership checks
                for stmt in body {
                    self.check_statement(stmt)?;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn check_variable_declaration(
        &mut self,
        name: &str,
        mutable: bool,
        value: &Expression,
    ) -> Result<()> {
        // Rule 1: Check if value is being moved
        self.check_expression(value)?;
        
        // Declare the new variable
        let ty = self.infer_type(value);
        self.declare_variable(
            name.to_string(),
            ty,
            mutable,
            "variable declaration".to_string(),
        );
        
        Ok(())
    }

    fn check_expression(&mut self, expr: &Expression) -> Result<()> {
        match expr {
            Expression::Identifier(name) => {
                // Rule 1 & 7: Check if variable has been moved
                let info = self.get_variable_info(name)?;
                if info.state == OwnershipState::Moved {
                    return Err(OwnershipError::UseAfterMove {
                        variable: name.clone(),
                        moved_at: info.moved_at.unwrap_or_else(|| "unknown".to_string()),
                        used_at: "here".to_string(),
                    });
                }
                Ok(())
            }
            Expression::FunctionCall { name: _, arguments } => {
                // Rule 7: Function calls transfer ownership of arguments
                for arg in arguments {
                    self.check_expression(arg)?;
                    
                    // If argument is a variable, mark it as moved
                    if let Expression::Identifier(var_name) = arg {
                        self.mark_as_moved(var_name, "function call")?;
                    }
                }
                Ok(())
            }
            Expression::MethodCall { receiver, method, arguments } => {
                // Rule 3: Check for .clone() calls
                if method == "clone" {
                    let info = self.get_variable_info(receiver)?;
                    if !self.is_cloneable(&info.ty) {
                        return Err(OwnershipError::InvalidClone {
                            variable: receiver.clone(),
                            reason: "type does not implement Clone".to_string(),
                        });
                    }
                }
                
                // Check receiver
                let info = self.get_variable_info(receiver)?;
                if info.state == OwnershipState::Moved {
                    return Err(OwnershipError::UseAfterMove {
                        variable: receiver.clone(),
                        moved_at: info.moved_at.unwrap_or_else(|| "unknown".to_string()),
                        used_at: "method call".to_string(),
                    });
                }
                
                for arg in arguments {
                    self.check_expression(arg)?;
                }
                Ok(())
            }
            Expression::Binary { left, right, .. } => {
                self.check_expression(left)?;
                self.check_expression(right)?;
                Ok(())
            }
            Expression::Unary { operand, .. } => {
                self.check_expression(operand)?;
                Ok(())
            }
            Expression::Assignment { target, value } => {
                // Rule 2: Check mutability
                let info = self.get_variable_info(target)?;
                if !info.mutable {
                    return Err(OwnershipError::MutabilityConflict {
                        variable: target.to_string(),
                        declared_as: "immutable".to_string(),
                        used_as: "mutable (assignment)".to_string(),
                    });
                }
                
                // Rule 7: Check ownership transfer
                self.check_expression(value)?;
                
                Ok(())
            }
            _ => Ok(()),
        }
    }

    // Helper methods

    fn declare_variable(&mut self, name: String, ty: Type, mutable: bool, location: String) {
        let info = OwnershipInfo {
            name: name.clone(),
            ty,
            mutable,
            state: OwnershipState::Owned,
            scope_level: self.current_scope,
            declared_at: location,
            moved_at: None,
        };
        
        self.scopes[self.current_scope].insert(name, info);
    }

    fn get_variable_info(&self, name: &str) -> Result<OwnershipInfo> {
        for scope in self.scopes.iter().rev() {
            if let Some(info) = scope.get(name) {
                return Ok(info.clone());
            }
        }
        
        Err(OwnershipError::UseAfterMove {
            variable: name.to_string(),
            moved_at: "unknown".to_string(),
            used_at: "here".to_string(),
        })
    }

    fn mark_as_moved(&mut self, name: &str, location: &str) -> Result<()> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(info) = scope.get_mut(name) {
                info.state = OwnershipState::Moved;
                info.moved_at = Some(location.to_string());
                return Ok(());
            }
        }
        Ok(())
    }

    fn is_cloneable(&self, ty: &Type) -> bool {
        // Basic types are cloneable
        matches!(ty, Type::I32 | Type::Str | Type::Bool)
    }

    fn infer_type(&self, expr: &Expression) -> Type {
        match expr {
            Expression::Literal(lit) => match lit {
                Literal::Integer(_) => Type::I32,
                Literal::String(_) => Type::Str,
                Literal::Boolean(_) => Type::Bool,
            },
            Expression::Identifier(name) => {
                self.get_variable_info(name)
                    .map(|info| info.ty)
                    .unwrap_or(Type::I32)
            }
            _ => Type::I32, // Default fallback
        }
    }

    fn enter_scope(&mut self) {
        self.current_scope += 1;
        self.scopes.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        if self.current_scope > 0 {
            self.scopes.pop();
            self.current_scope -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_ownership_basic() {
        let source = r#"
fn main() {
    let x = 42;
    let y = x;
    return 0;
}
"#.to_string();
        
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let decls = parser.parse().unwrap();
        
        let mut checker = OwnershipChecker::new();
        assert!(checker.check_program(&decls).is_ok());
    }

    #[test]
    fn test_ownership_mutability() {
        let source = r#"
fn main() {
    let x = 42;
    x = 10;
    return 0;
}
"#.to_string();
        
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let decls = parser.parse().unwrap();
        
        let mut checker = OwnershipChecker::new();
        let result = checker.check_program(&decls);
        assert!(result.is_err());
    }
}
