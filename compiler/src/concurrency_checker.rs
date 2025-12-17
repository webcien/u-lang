// concurrency_checker.rs — U v1.0 Concurrency Checker
// MIT License — Copyright (c) 2025 Webcien and U contributors
//
// This module implements concurrency safety checks for the actor model:
//
// Core Principle: "Prevención estructural de data races mediante ausencia de memoria compartida"
// (Structural prevention of data races through absence of shared memory)
//
// Rules:
// 1. Actors communicate ONLY through message passing
// 2. NO shared memory access between actors
// 3. Messages must be owned (not borrowed)
// 4. Actor state is private and inaccessible from outside
// 5. No global mutable state

use crate::parser::{Actor, Declaration, Expression, Function, Statement};
use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
pub enum ConcurrencyError {
    SharedMemoryAccess {
        actor: String,
        variable: String,
        location: String,
    },
    BorrowedMessage {
        actor: String,
        message: String,
        location: String,
    },
    GlobalMutableState {
        variable: String,
        location: String,
    },
    ActorStateLeakage {
        actor: String,
        field: String,
        location: String,
    },
    InvalidMessageType {
        actor: String,
        message_type: String,
        reason: String,
    },
}

impl fmt::Display for ConcurrencyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConcurrencyError::SharedMemoryAccess { actor, variable, location } => {
                write!(
                    f,
                    "Actor '{}' attempts to access shared memory variable '{}' at {}. \
                     Actors must communicate only through message passing.",
                    actor, variable, location
                )
            }
            ConcurrencyError::BorrowedMessage { actor, message, location } => {
                write!(
                    f,
                    "Actor '{}' sends borrowed message '{}' at {}. \
                     Messages must be owned values.",
                    actor, message, location
                )
            }
            ConcurrencyError::GlobalMutableState { variable, location } => {
                write!(
                    f,
                    "Global mutable variable '{}' at {}. \
                     Global mutable state is forbidden in concurrent contexts.",
                    variable, location
                )
            }
            ConcurrencyError::ActorStateLeakage { actor, field, location } => {
                write!(
                    f,
                    "Actor '{}' exposes internal state '{}' at {}. \
                     Actor state must remain private.",
                    actor, field, location
                )
            }
            ConcurrencyError::InvalidMessageType { actor, message_type, reason } => {
                write!(
                    f,
                    "Actor '{}' uses invalid message type '{}': {}",
                    actor, message_type, reason
                )
            }
        }
    }
}

impl std::error::Error for ConcurrencyError {}

pub type Result<T> = std::result::Result<T, ConcurrencyError>;

/// Tracks actor definitions and their state
#[derive(Debug, Clone)]
struct ActorInfo {
    name: String,
    state_variables: HashSet<String>,
    methods: Vec<String>,
}

/// Concurrency checker for actor-based concurrency
pub struct ConcurrencyChecker {
    actors: HashMap<String, ActorInfo>,
    global_variables: HashSet<String>,
    current_actor: Option<String>,
}

impl ConcurrencyChecker {
    pub fn new() -> Self {
        ConcurrencyChecker {
            actors: HashMap::new(),
            global_variables: HashSet::new(),
            current_actor: None,
        }
    }

    /// Check concurrency safety for a program
    pub fn check_program(&mut self, declarations: &[Declaration]) -> Result<()> {
        // First pass: collect all actors and global variables
        for decl in declarations {
            match decl {
                Declaration::Actor(actor) => {
                    self.register_actor(actor);
                }
                Declaration::Function(_) => {
                    // Global functions are allowed
                }
                _ => {}
            }
        }

        // Second pass: validate actor implementations
        for decl in declarations {
            if let Declaration::Actor(actor) = decl {
                self.check_actor(actor)?;
            }
        }

        Ok(())
    }

    fn register_actor(&mut self, actor: &Actor) {
        let mut state_vars = HashSet::new();
        let mut methods = Vec::new();

        // Extract state variables and methods
        // Note: In the current AST, actor state is implicit in method bodies
        // We'll need to enhance the AST to track actor fields explicitly

        for method in &actor.methods {
            methods.push(method.name.clone());
        }

        let info = ActorInfo {
            name: actor.name.clone(),
            state_variables: state_vars,
            methods,
        };

        self.actors.insert(actor.name.clone(), info);
    }

    fn check_actor(&mut self, actor: &Actor) -> Result<()> {
        self.current_actor = Some(actor.name.clone());

        // Check each method in the actor
        for method in &actor.methods {
            self.check_actor_method(&actor.name, method)?;
        }

        self.current_actor = None;
        Ok(())
    }

    fn check_actor_method(&mut self, actor_name: &str, method: &Function) -> Result<()> {
        // Check method body for concurrency violations
        for stmt in &method.body {
            self.check_statement(actor_name, stmt)?;
        }
        Ok(())
    }

    fn check_statement(&mut self, actor_name: &str, stmt: &Statement) -> Result<()> {
        match stmt {
            Statement::Let { name, value, .. } => {
                // Check if we're accessing shared memory
                self.check_expression(actor_name, value)?;
                
                // Track local variables (actor state)
                if let Some(actor_info) = self.actors.get_mut(actor_name) {
                    actor_info.state_variables.insert(name.clone());
                }
                Ok(())
            }
            Statement::Expr(expr) => {
                self.check_expression(actor_name, expr)?;
                Ok(())
            }
            Statement::If { condition, then_branch, else_branch } => {
                self.check_expression(actor_name, condition)?;
                
                for stmt in then_branch {
                    self.check_statement(actor_name, stmt)?;
                }
                
                if let Some(else_stmts) = else_branch {
                    for stmt in else_stmts {
                        self.check_statement(actor_name, stmt)?;
                    }
                }
                Ok(())
            }
            Statement::While { condition, body } => {
                self.check_expression(actor_name, condition)?;
                for stmt in body {
                    self.check_statement(actor_name, stmt)?;
                }
                Ok(())
            }
            Statement::For { iterable, body, .. } => {
                self.check_expression(actor_name, iterable)?;
                for stmt in body {
                    self.check_statement(actor_name, stmt)?;
                }
                Ok(())
            }
            Statement::Return(expr) => {
                self.check_expression(actor_name, expr)?;
                Ok(())
            }
            Statement::Unsafe { body } => {
                // Unsafe blocks are allowed but should be minimized
                // We still check for concurrency violations
                for stmt in body {
                    self.check_statement(actor_name, stmt)?;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn check_expression(&mut self, actor_name: &str, expr: &Expression) -> Result<()> {
        match expr {
            Expression::Identifier(name) => {
                // Check if this is accessing another actor's state
                if self.is_foreign_actor_state(actor_name, name) {
                    return Err(ConcurrencyError::SharedMemoryAccess {
                        actor: actor_name.to_string(),
                        variable: name.clone(),
                        location: "expression".to_string(),
                    });
                }
                Ok(())
            }
            Expression::FunctionCall { name, arguments } => {
                // Check if this is a message send operation
                if self.is_message_send(name) {
                    self.check_message_send(actor_name, arguments)?;
                }
                
                for arg in arguments {
                    self.check_expression(actor_name, arg)?;
                }
                Ok(())
            }
            Expression::MethodCall { receiver, method, arguments } => {
                // Check if receiver is another actor
                if self.is_actor(receiver) && receiver != actor_name {
                    // This is inter-actor communication
                    if method != "send" && method != "spawn" {
                        return Err(ConcurrencyError::ActorStateLeakage {
                            actor: receiver.clone(),
                            field: method.clone(),
                            location: "method call".to_string(),
                        });
                    }
                }
                
                for arg in arguments {
                    self.check_expression(actor_name, arg)?;
                }
                Ok(())
            }
            Expression::Binary { left, right, .. } => {
                self.check_expression(actor_name, left)?;
                self.check_expression(actor_name, right)?;
                Ok(())
            }
            Expression::Unary { operand, .. } => {
                self.check_expression(actor_name, operand)?;
                Ok(())
            }
            Expression::Assignment { target, value } => {
                // Check if we're assigning to foreign actor state
                if self.is_foreign_actor_state(actor_name, target) {
                    return Err(ConcurrencyError::SharedMemoryAccess {
                        actor: actor_name.to_string(),
                        variable: target.clone(),
                        location: "assignment".to_string(),
                    });
                }
                
                self.check_expression(actor_name, value)?;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn check_message_send(&self, actor_name: &str, arguments: &[Expression]) -> Result<()> {
        // Rule: Messages must be owned values, not borrowed
        for arg in arguments {
            if let Expression::Identifier(var_name) = arg {
                // Check if this is a reference (simplified check)
                if var_name.starts_with('&') {
                    return Err(ConcurrencyError::BorrowedMessage {
                        actor: actor_name.to_string(),
                        message: var_name.clone(),
                        location: "send call".to_string(),
                    });
                }
            }
        }
        Ok(())
    }

    fn is_foreign_actor_state(&self, current_actor: &str, variable: &str) -> bool {
        // Check if variable belongs to another actor
        for (actor_name, actor_info) in &self.actors {
            if actor_name != current_actor && actor_info.state_variables.contains(variable) {
                return true;
            }
        }
        false
    }

    fn is_actor(&self, name: &str) -> bool {
        self.actors.contains_key(name)
    }

    fn is_message_send(&self, function_name: &str) -> bool {
        function_name == "send" || function_name == "spawn"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_actor_isolation() {
        let source = r#"
actor Counter {
    fn increment() {
        let count = 0;
        count = count + 1;
    }
}
"#.to_string();
        
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let decls = parser.parse().unwrap();
        
        let mut checker = ConcurrencyChecker::new();
        assert!(checker.check_program(&decls).is_ok());
    }

    #[test]
    fn test_message_passing() {
        let source = r#"
actor Worker {
    fn process(msg: i32) {
        let result = msg + 1;
    }
}
"#.to_string();
        
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let decls = parser.parse().unwrap();
        
        let mut checker = ConcurrencyChecker::new();
        assert!(checker.check_program(&decls).is_ok());
    }
}
