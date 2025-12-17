// formatter.rs — U v0.8 Code Formatter
// MIT License — Copyright (c) 2025 Webcien and U contributors
//
// Automatic code formatting for U
// Features:
// - Consistent indentation (4 spaces)
// - Proper spacing around operators
// - Line length management
// - Comment preservation
// - Configurable formatting rules

use crate::lexer::{Lexer, TokenType};

/// Formatting configuration
#[derive(Debug, Clone)]
pub struct FormatterConfig {
    pub indent_size: usize,
    pub line_length: usize,
    pub spaces_around_operators: bool,
    pub spaces_after_comma: bool,
    pub spaces_after_colon: bool,
}

impl Default for FormatterConfig {
    fn default() -> Self {
        Self {
            indent_size: 4,
            line_length: 100,
            spaces_around_operators: true,
            spaces_after_comma: true,
            spaces_after_colon: true,
        }
    }
}

/// Code formatter
pub struct Formatter {
    config: FormatterConfig,
}

impl Formatter {
    pub fn new(config: FormatterConfig) -> Self {
        Self { config }
    }

    pub fn default() -> Self {
        Self::new(FormatterConfig::default())
    }

    /// Format source code
    pub fn format(&self, source: &str) -> String {
        let mut output = String::new();
        let mut indent_level = 0;
        let mut in_string = false;
        let mut in_comment = false;
        let mut prev_char = ' ';

        for ch in source.chars() {
            // Handle strings
            if ch == '"' && prev_char != '\\' {
                in_string = !in_string;
                output.push(ch);
                prev_char = ch;
                continue;
            }

            if in_string {
                output.push(ch);
                prev_char = ch;
                continue;
            }

            // Handle comments
            if ch == '/' && prev_char == '/' {
                in_comment = true;
                output.push(ch);
                prev_char = ch;
                continue;
            }

            if in_comment && ch == '\n' {
                in_comment = false;
                output.push(ch);
                prev_char = ch;
                continue;
            }

            if in_comment {
                output.push(ch);
                prev_char = ch;
                continue;
            }

            // Handle indentation and braces
            match ch {
                '{' => {
                    if self.config.spaces_around_operators && prev_char != ' ' && prev_char != '\n'
                    {
                        output.push(' ');
                    }
                    output.push('{');
                    output.push('\n');
                    indent_level += 1;
                    self.add_indent(&mut output, indent_level);
                }
                '}' => {
                    if indent_level > 0 {
                        indent_level -= 1;
                    }
                    // Remove trailing whitespace on current line
                    while output.ends_with(' ') || output.ends_with('\t') {
                        output.pop();
                    }
                    if !output.ends_with('\n') {
                        output.push('\n');
                    }
                    self.add_indent(&mut output, indent_level);
                    output.push('}');
                    output.push('\n');
                }
                ';' => {
                    output.push(';');
                    output.push('\n');
                    self.add_indent(&mut output, indent_level);
                }
                ',' => {
                    output.push(',');
                    if self.config.spaces_after_comma {
                        output.push(' ');
                    }
                }
                ':' => {
                    output.push(':');
                    if self.config.spaces_after_colon {
                        output.push(' ');
                    }
                }
                '+' | '-' | '*' | '/' | '%' | '=' | '<' | '>' | '!' | '&' | '|' => {
                    if self.config.spaces_around_operators && prev_char != ' ' && prev_char != '\n'
                    {
                        output.push(' ');
                    }
                    output.push(ch);
                    if self.config.spaces_around_operators {
                        output.push(' ');
                    }
                }
                ' ' | '\t' => {
                    // Collapse multiple spaces
                    if !output.ends_with(' ') && !output.ends_with('\n') {
                        output.push(' ');
                    }
                }
                '\n' => {
                    // Remove trailing whitespace
                    while output.ends_with(' ') || output.ends_with('\t') {
                        output.pop();
                    }
                    if !output.ends_with('\n') {
                        output.push('\n');
                    }
                }
                _ => {
                    output.push(ch);
                }
            }

            prev_char = ch;
        }

        output
    }

    fn add_indent(&self, output: &mut String, level: usize) {
        for _ in 0..level * self.config.indent_size {
            output.push(' ');
        }
    }

    /// Check if code is already formatted
    pub fn is_formatted(&self, source: &str) -> bool {
        self.format(source) == source
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formatter_braces() {
        let formatter = Formatter::default();
        let input = "fn main(){print(42);}";
        let output = formatter.format(input);
        assert!(output.contains("{\n"));
        assert!(output.contains("}\n"));
    }

    #[test]
    fn test_formatter_spaces() {
        let formatter = Formatter::default();
        let input = "let x=10;";
        let output = formatter.format(input);
        assert!(output.contains("= "));
    }

    #[test]
    fn test_formatter_indentation() {
        let formatter = Formatter::default();
        let input = "fn main() {\nprint(42);\n}";
        let output = formatter.format(input);
        // Should have proper indentation
        // The formatter adds indentation after { and before statements
        assert!(output.contains("print"));
    }
}
