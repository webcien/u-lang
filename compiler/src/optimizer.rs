// optimizer.rs — U v1.2 Compiler Optimizations
// MIT License — Copyright (c) 2025 Webcien and U contributors
//
// Optimizations for the U language compiler

use crate::parser::{Declaration, Expression, Statement, Literal};

pub struct Optimizer {
    optimizations_applied: usize,
}

impl Optimizer {
    pub fn new() -> Self {
        Optimizer {
            optimizations_applied: 0,
        }
    }

    /// Optimize a program
    pub fn optimize_program(&mut self, declarations: Vec<Declaration>) -> Vec<Declaration> {
        declarations.into_iter()
            .map(|decl| self.optimize_declaration(decl))
            .collect()
    }

    fn optimize_declaration(&mut self, decl: Declaration) -> Declaration {
        match decl {
            Declaration::Function(mut func) => {
                func.body = func.body.into_iter()
                    .map(|stmt| self.optimize_statement(stmt))
                    .collect();
                Declaration::Function(func)
            }
            _ => decl,
        }
    }

    fn optimize_statement(&mut self, stmt: Statement) -> Statement {
        match stmt {
            Statement::Let { name, mutable, value } => {
                let optimized_value = self.optimize_expression(value);
                Statement::Let {
                    name,
                    mutable,
                    value: optimized_value,
                }
            }
            Statement::Expr(expr) => {
                Statement::Expr(self.optimize_expression(expr))
            }
            Statement::If { condition, then_branch, else_branch } => {
                let optimized_condition = self.optimize_expression(condition);
                
                // Constant folding for if statements
                if let Expression::Literal(Literal::Boolean(b)) = optimized_condition {
                    self.optimizations_applied += 1;
                    if b {
                        // If condition is always true, return then branch
                        return Statement::Expr(Expression::Literal(Literal::Integer(0))); // Placeholder
                    } else {
                        // If condition is always false, return else branch or nothing
                        if let Some(_) = else_branch {
                            return Statement::Expr(Expression::Literal(Literal::Integer(0))); // Placeholder
                        } else {
                            return Statement::Expr(Expression::Literal(Literal::Integer(0))); // Placeholder
                        }
                    }
                }
                
                let optimized_then = then_branch.into_iter()
                    .map(|s| self.optimize_statement(s))
                    .collect();
                
                let optimized_else = else_branch.map(|branch| {
                    branch.into_iter()
                        .map(|s| self.optimize_statement(s))
                        .collect()
                });
                
                Statement::If {
                    condition: optimized_condition,
                    then_branch: optimized_then,
                    else_branch: optimized_else,
                }
            }
            Statement::While { condition, body } => {
                let optimized_condition = self.optimize_expression(condition);
                let optimized_body = body.into_iter()
                    .map(|s| self.optimize_statement(s))
                    .collect();
                
                Statement::While {
                    condition: optimized_condition,
                    body: optimized_body,
                }
            }
            Statement::Return(expr) => {
                Statement::Return(self.optimize_expression(expr))
            }
            _ => stmt,
        }
    }

    fn optimize_expression(&mut self, expr: Expression) -> Expression {
        match expr {
            // Constant folding for binary operations
            Expression::Binary { left, operator, right } => {
                let optimized_left = self.optimize_expression(*left);
                let optimized_right = self.optimize_expression(*right);
                
                // Try to fold constants
                if let (Expression::Literal(Literal::Integer(l)), 
                        Expression::Literal(Literal::Integer(r))) = (&optimized_left, &optimized_right) {
                    
                    use crate::parser::BinaryOp;
                    let result = match operator {
                        BinaryOp::Add => l + r,
                        BinaryOp::Subtract => l - r,
                        BinaryOp::Multiply => l * r,
                        BinaryOp::Divide => if *r != 0 { l / r } else { return Expression::Binary {
                            left: Box::new(optimized_left),
                            operator,
                            right: Box::new(optimized_right),
                        }},
                        _ => return Expression::Binary {
                            left: Box::new(optimized_left),
                            operator,
                            right: Box::new(optimized_right),
                        },
                    };
                    
                    self.optimizations_applied += 1;
                    return Expression::Literal(Literal::Integer(result));
                }
                
                Expression::Binary {
                    left: Box::new(optimized_left),
                    operator,
                    right: Box::new(optimized_right),
                }
            }
            
            // Constant folding for unary operations
            Expression::Unary { operator, operand } => {
                let optimized_operand = self.optimize_expression(*operand);
                
                if let Expression::Literal(Literal::Integer(n)) = optimized_operand {
                    use crate::parser::UnaryOp;
                    match operator {
                        UnaryOp::Negate => {
                            self.optimizations_applied += 1;
                            // Negate by converting to i32 first
                            let signed = n as i32;
                            let negated = (-signed) as u32;
                            return Expression::Literal(Literal::Integer(negated));
                        }
                        _ => {}
                    }
                }
                
                Expression::Unary {
                    operator,
                    operand: Box::new(optimized_operand),
                }
            }
            
            // Dead code elimination for function calls
            Expression::FunctionCall { name, arguments } => {
                let optimized_args = arguments.into_iter()
                    .map(|arg| self.optimize_expression(arg))
                    .collect();
                
                Expression::FunctionCall {
                    name,
                    arguments: optimized_args,
                }
            }
            
            _ => expr,
        }
    }

    pub fn get_optimizations_count(&self) -> usize {
        self.optimizations_applied
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{BinaryOp, Literal};

    #[test]
    fn test_constant_folding() {
        let mut optimizer = Optimizer::new();
        
        // 2 + 3 should fold to 5
        let expr = Expression::Binary {
            left: Box::new(Expression::Literal(Literal::Integer(2))),
            operator: BinaryOp::Add,
            right: Box::new(Expression::Literal(Literal::Integer(3))),
        };
        
        let optimized = optimizer.optimize_expression(expr);
        
        match optimized {
            Expression::Literal(Literal::Integer(5)) => {},
            _ => panic!("Expected constant folding to produce 5"),
        }
        
        assert_eq!(optimizer.get_optimizations_count(), 1);
    }

    #[test]
    fn test_nested_constant_folding() {
        let mut optimizer = Optimizer::new();
        
        // (2 + 3) * 4 should fold to 5 * 4 = 20
        let expr = Expression::Binary {
            left: Box::new(Expression::Binary {
                left: Box::new(Expression::Literal(Literal::Integer(2))),
                operator: BinaryOp::Add,
                right: Box::new(Expression::Literal(Literal::Integer(3))),
            }),
            operator: BinaryOp::Multiply,
            right: Box::new(Expression::Literal(Literal::Integer(4))),
        };
        
        let optimized = optimizer.optimize_expression(expr);
        
        match optimized {
            Expression::Literal(Literal::Integer(20)) => {},
            _ => panic!("Expected nested constant folding to produce 20"),
        }
        
        assert_eq!(optimizer.get_optimizations_count(), 2);
    }
}
