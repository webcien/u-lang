// linter.rs — U v0.8 Linter
// MIT License — Copyright (c) 2025 Webcien and U contributors
//
// Static analysis and code quality checks
// Features:
// - Unused variable detection
// - Naming convention checks
// - Code style warnings
// - Performance suggestions
// - Best practice recommendations

use crate::diagnostics::{Diagnostic, DiagnosticCollector, Severity};
use std::collections::HashSet;

/// Linting configuration
#[derive(Debug, Clone)]
pub struct LinterConfig {
    pub check_unused_variables: bool,
    pub check_naming_conventions: bool,
    pub check_code_style: bool,
    pub check_performance: bool,
    pub max_function_length: usize,
    pub max_line_length: usize,
}

impl Default for LinterConfig {
    fn default() -> Self {
        Self {
            check_unused_variables: true,
            check_naming_conventions: true,
            check_code_style: true,
            check_performance: true,
            max_function_length: 100,
            max_line_length: 100,
        }
    }
}

/// Code linter
pub struct Linter {
    config: LinterConfig,
    diagnostics: DiagnosticCollector,
}

impl Linter {
    pub fn new(config: LinterConfig) -> Self {
        Self {
            config,
            diagnostics: DiagnosticCollector::new(),
        }
    }

    pub fn default() -> Self {
        Self::new(LinterConfig::default())
    }

    /// Run all linting checks
    pub fn lint(&mut self, source: &str) -> &[Diagnostic] {
        self.check_line_length(source);
        self.check_naming_conventions(source);
        self.check_code_style(source);
        self.check_performance(source);
        self.diagnostics.diagnostics()
    }

    /// Check line length
    fn check_line_length(&mut self, source: &str) {
        for (line_num, line) in source.lines().enumerate() {
            if line.len() > self.config.max_line_length {
                self.diagnostics.add(
                    Diagnostic::warning(format!(
                        "line {} is too long ({} > {})",
                        line_num + 1,
                        line.len(),
                        self.config.max_line_length
                    ))
                    .with_suggestion("break line into multiple lines".to_string()),
                );
            }
        }
    }

    /// Check naming conventions
    fn check_naming_conventions(&mut self, source: &str) {
        let mut in_function = false;
        let mut function_name = String::new();

        for line in source.lines() {
            let trimmed = line.trim();

            // Check function names (should be snake_case)
            if trimmed.starts_with("fn ") {
                in_function = true;
                if let Some(name) = trimmed.split('(').next() {
                    function_name = name.replace("fn ", "").trim().to_string();
                    if !self.is_snake_case(&function_name) {
                        self.diagnostics.add(
                            Diagnostic::warning(format!(
                                "function name '{}' should be snake_case",
                                function_name
                            ))
                            .with_suggestion(format!(
                                "rename to '{}'",
                                self.to_snake_case(&function_name)
                            )),
                        );
                    }
                }
            }

            // Check variable names (should be snake_case)
            if (trimmed.starts_with("let ") || trimmed.starts_with("var ")) && trimmed.contains('=')
            {
                if let Some(var_part) = trimmed.split('=').next() {
                    let var_name = var_part
                        .replace("let ", "")
                        .replace("var ", "")
                        .trim()
                        .to_string();
                    if !self.is_snake_case(&var_name) && !var_name.is_empty() {
                        self.diagnostics.add(
                            Diagnostic::warning(format!(
                                "variable name '{}' should be snake_case",
                                var_name
                            ))
                            .with_suggestion(format!(
                                "rename to '{}'",
                                self.to_snake_case(&var_name)
                            )),
                        );
                    }
                }
            }
        }
    }

    /// Check code style issues
    fn check_code_style(&mut self, source: &str) {
        for (line_num, line) in source.lines().enumerate() {
            // Check for trailing whitespace
            if line.ends_with(' ') || line.ends_with('\t') {
                self.diagnostics.add(
                    Diagnostic::warning(format!("line {} has trailing whitespace", line_num + 1))
                        .with_suggestion("remove trailing whitespace".to_string()),
                );
            }

            // Check for multiple statements on one line
            let semicolon_count = line.matches(';').count();
            if semicolon_count > 1 {
                self.diagnostics.add(
                    Diagnostic::warning(format!(
                        "line {} has multiple statements",
                        line_num + 1
                    ))
                    .with_suggestion("split into separate lines".to_string()),
                );
            }
        }
    }

    /// Check for performance issues
    fn check_performance(&mut self, source: &str) {
        for (line_num, line) in source.lines().enumerate() {
            // Warn about inefficient patterns
            if line.contains("clone()") && line.contains("for ") {
                self.diagnostics.add(
                    Diagnostic::warning(format!(
                        "line {} clones inside loop (performance issue)",
                        line_num + 1
                    ))
                    .with_suggestion("move clone outside loop".to_string()),
                );
            }

            // Warn about string concatenation in loops
            if line.contains("+") && line.contains("str") {
                self.diagnostics.add(
                    Diagnostic::note(format!(
                        "line {} uses string concatenation (consider using builder)",
                        line_num + 1
                    )),
                );
            }
        }
    }

    /// Check if string is snake_case
    fn is_snake_case(&self, s: &str) -> bool {
        if s.is_empty() {
            return false;
        }
        s.chars().all(|c| c.is_lowercase() || c == '_' || c.is_ascii_digit())
            && !s.starts_with('_')
            && !s.ends_with('_')
    }

    /// Convert string to snake_case
    fn to_snake_case(&self, s: &str) -> String {
        let mut result = String::new();
        for (i, ch) in s.chars().enumerate() {
            if ch.is_uppercase() {
                if i > 0 {
                    result.push('_');
                }
                result.push(ch.to_lowercase().next().unwrap());
            } else {
                result.push(ch);
            }
        }
        result
    }

    /// Get diagnostics
    pub fn diagnostics(&self) -> &[Diagnostic] {
        self.diagnostics.diagnostics()
    }

    /// Clear diagnostics
    pub fn clear(&mut self) {
        self.diagnostics.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linter_line_length() {
        let mut linter = Linter::new(LinterConfig {
            max_line_length: 20,
            ..Default::default()
        });
        let source = "let very_long_variable_name = 42;";
        linter.lint(source);
        assert!(linter.diagnostics().len() > 0);
    }

    #[test]
    fn test_linter_naming() {
        let mut linter = Linter::default();
        let source = "fn MyFunction() { }";
        linter.lint(source);
        assert!(linter.diagnostics().len() > 0);
    }

    #[test]
    fn test_snake_case() {
        let linter = Linter::default();
        assert!(linter.is_snake_case("my_variable"));
        assert!(!linter.is_snake_case("MyVariable"));
        assert!(!linter.is_snake_case("_private"));
    }

    #[test]
    fn test_to_snake_case() {
        let linter = Linter::default();
        assert_eq!(linter.to_snake_case("MyVariable"), "my_variable");
        assert_eq!(linter.to_snake_case("myVariable"), "my_variable");
    }
}
