// macro_expander.rs — Macro expansion system for U Language
// MIT License — Copyright (c) 2025 Webcien and U contributors

use crate::parser::*;
use std::collections::HashMap;

/// Represents a macro definition
#[derive(Debug, Clone)]
pub struct MacroDefinition {
    pub name: String,
    pub rules: Vec<MacroRule>,
}

/// Represents a single macro rule (pattern => expansion)
#[derive(Debug, Clone)]
pub struct MacroRule {
    pub pattern: Vec<MacroToken>,
    pub expansion: Vec<MacroToken>,
}

/// Represents a token in a macro pattern or expansion
#[derive(Debug, Clone, PartialEq)]
pub enum MacroToken {
    Ident(String),
    Literal(String),
    Punct(char),
    Designator(String, String), // ($name:designator)
    Repetition(Vec<MacroToken>, char), // $(...)*  or $(...)+ (char is '*' or '+')
}

/// Macro expander
pub struct MacroExpander {
    macros: HashMap<String, MacroDefinition>,
    hygiene_counter: usize,
}

impl MacroExpander {
    pub fn new() -> Self {
        let mut expander = MacroExpander {
            macros: HashMap::new(),
            hygiene_counter: 0,
        };

        // Register built-in macros
        expander.register_builtin_macros();
        expander
    }

    /// Register built-in macros
    fn register_builtin_macros(&mut self) {
        // println! macro
        self.register_macro(MacroDefinition {
            name: "println".to_string(),
            rules: vec![MacroRule {
                pattern: vec![MacroToken::Designator("fmt".to_string(), "expr".to_string())],
                expansion: vec![
                    MacroToken::Ident("printf".to_string()),
                    MacroToken::Punct('('),
                    MacroToken::Ident("fmt".to_string()),
                    MacroToken::Punct(','),
                    MacroToken::Literal("\"\\n\"".to_string()),
                    MacroToken::Punct(')'),
                ],
            }],
        });

        // assert! macro
        self.register_macro(MacroDefinition {
            name: "assert".to_string(),
            rules: vec![MacroRule {
                pattern: vec![MacroToken::Designator("cond".to_string(), "expr".to_string())],
                expansion: vec![
                    MacroToken::Ident("if".to_string()),
                    MacroToken::Punct('('),
                    MacroToken::Punct('!'),
                    MacroToken::Ident("cond".to_string()),
                    MacroToken::Punct(')'),
                    MacroToken::Punct('{'),
                    MacroToken::Ident("panic".to_string()),
                    MacroToken::Punct('('),
                    MacroToken::Literal("\"assertion failed\"".to_string()),
                    MacroToken::Punct(')'),
                    MacroToken::Punct(';'),
                    MacroToken::Punct('}'),
                ],
            }],
        });

        // vec! macro
        self.register_macro(MacroDefinition {
            name: "vec".to_string(),
            rules: vec![MacroRule {
                pattern: vec![MacroToken::Repetition(
                    vec![MacroToken::Designator("elem".to_string(), "expr".to_string())],
                    '*',
                )],
                expansion: vec![
                    MacroToken::Ident("Vec_from_array".to_string()),
                    MacroToken::Punct('('),
                    MacroToken::Repetition(
                        vec![MacroToken::Ident("elem".to_string())],
                        '*',
                    ),
                    MacroToken::Punct(')'),
                ],
            }],
        });
    }

    /// Register a macro
    pub fn register_macro(&mut self, macro_def: MacroDefinition) {
        self.macros.insert(macro_def.name.clone(), macro_def);
    }

    /// Expand a macro call
    pub fn expand_macro(&mut self, name: &str, args: Vec<String>) -> Result<String, String> {
        if let Some(macro_def) = self.macros.get(name) {
            // For simplicity, we'll just use the first rule
            if let Some(rule) = macro_def.rules.first() {
                // Simple expansion: replace designators with arguments
                let mut expansion = String::new();
                for token in &rule.expansion {
                    match token {
                        MacroToken::Ident(id) => {
                            // Check if this is a designator that should be replaced
                            if let Some(arg_idx) = rule.pattern.iter().position(|p| {
                                if let MacroToken::Designator(name, _) = p {
                                    name == id
                                } else {
                                    false
                                }
                            }) {
                                if arg_idx < args.len() {
                                    expansion.push_str(&args[arg_idx]);
                                } else {
                                    expansion.push_str(id);
                                }
                            } else {
                                expansion.push_str(id);
                            }
                        }
                        MacroToken::Literal(lit) => expansion.push_str(lit),
                        MacroToken::Punct(ch) => expansion.push(*ch),
                        MacroToken::Repetition(tokens, _) => {
                            // Handle repetition by expanding each argument
                            for (i, arg) in args.iter().enumerate() {
                                if i > 0 {
                                    expansion.push_str(", ");
                                }
                                expansion.push_str(arg);
                            }
                        }
                        _ => {}
                    }
                }
                Ok(expansion)
            } else {
                Err(format!("Macro '{}' has no rules", name))
            }
        } else {
            Err(format!("Macro '{}' not found", name))
        }
    }

    /// Expand all macros in a program
    pub fn expand_program(&mut self, declarations: Vec<Declaration>) -> Result<Vec<Declaration>, String> {
        // For now, we'll just return the declarations as-is
        // In a full implementation, we would traverse the AST and expand macro calls
        Ok(declarations)
    }

    /// Generate a unique identifier for hygiene
    fn generate_unique_ident(&mut self, base: &str) -> String {
        self.hygiene_counter += 1;
        format!("{}__macro{}", base, self.hygiene_counter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_println_macro() {
        let mut expander = MacroExpander::new();
        let result = expander.expand_macro("println", vec!["\"Hello\"".to_string()]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_assert_macro() {
        let mut expander = MacroExpander::new();
        let result = expander.expand_macro("assert", vec!["x == 10".to_string()]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_vec_macro() {
        let mut expander = MacroExpander::new();
        let result = expander.expand_macro("vec", vec!["1".to_string(), "2".to_string(), "3".to_string()]);
        assert!(result.is_ok());
    }
}
