// lexer.rs — U v0.6
// MIT License — Copyright (c) 2025 Webcien and U contributors

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Keywords
    Fn,
    Actor,
    Let,
    Var,
    Return,
    Await,
    For,
    While,
    In,
    Break,
    Continue,
    If,
    Else,
    Extern,
    Unsafe,
    Ui,
    Child,
    Children,

    // Literals
    Identifier(String),
    StringLiteral(String),
    IntegerLiteral(u32),
    // Symbols
    LeftParen,      // (
    RightParen,     // )
    LeftBrace,      // {
    RightBrace,     // }
    LeftBracket,    // [
    RightBracket,   // ]
    Comma,          // ,
    Semicolon,      // ;
    Dot,            // .
    Arrow,          // ->
    Colon,          // :
    Equal,          // =
    Plus,           // +
    Minus,          // -
    Star,           // *
    Slash,          // /
    Percent,        // %
    DoubleEqual,    // ==
    NotEqual,       // !=
    LessEqual,      // <=
    GreaterEqual,   // >=
    And,            // &&
    Or,             // ||
    Not,            // !

    // Types
    I32,
    Str,
    Bool,
    Ptr,
    Option,
    Result,
    LessThan,       // <
    GreaterThan,    // >
    CommaType,      // , used within generic types

    // End of file
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenType,
    pub line: usize,
    pub column: usize,
}

pub struct Lexer {
    source: String,
    current: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source,
            current: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while !self.is_at_end() {
            tokens.push(self.scan_token());
        }
        tokens.push(Token {
            kind: TokenType::Eof,
            line: self.line,
            column: self.column,
        });
        tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap_or('\0');
        self.current += 1;
        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        c
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap_or('\0')
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap_or('\0')
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.peek() != expected {
            false
        } else {
            self.advance();
            true
        }
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                ' ' | '\r' | '\t' | '\n' => {
                    self.advance();
                }
                '/' if self.peek_next() == '/' => {
                    // Line comment: skip until end of line
                    while !self.is_at_end() && self.peek() != '\n' {
                        self.advance();
                    }
                }
                _ => break,
            }
        }
    }

    fn identifier(&mut self) -> String {
        let start = self.current - 1;
        while !self.is_at_end() {
            let c = self.peek();
            if c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }
        self.source[start..self.current].to_string()
    }

    fn string(&mut self) -> String {
        let mut value = String::new();
        while !self.is_at_end() && self.peek() != '"' {
            let c = self.advance();
            if c == '\\' {
                // Basic escape support (optional in MVP)
                let next = self.advance();
                match next {
                    '"' | '\\' => value.push(next),
                    'n' => value.push('\n'),
                    _ => value.push(next), // or report error
                }
            } else {
                value.push(c);
            }
        }
        if self.is_at_end() {
            // Error: unclosed string (will be handled in parser or checker)
        } else {
            self.advance(); // consume the '"'
        }
        value
    }

    fn number(&mut self) -> u32 {
        let start = self.current - 1;
        while !self.is_at_end() && self.peek().is_ascii_digit() {
            self.advance();
        }
        let num_str = &self.source[start..self.current];
        num_str.parse().unwrap_or(0)
    }

    fn scan_token(&mut self) -> Token {
        self.skip_whitespace();

        let line = self.line;
        let column = self.column;

        if self.is_at_end() {
            return Token {
                kind: TokenType::Eof,
                line,
                column,
            };
        }

        let c = self.advance();

        match c {
            '(' => Token {
                kind: TokenType::LeftParen,
                line,
                column,
            },
            ')' => Token {
                kind: TokenType::RightParen,
                line,
                column,
            },
            '{' => Token {
                kind: TokenType::LeftBrace,
                line,
                column,
            },
            '}' => Token {
                kind: TokenType::RightBrace,
                line,
                column,
            },
            '[' => Token {
                kind: TokenType::LeftBracket,
                line,
                column,
            },
            ']' => Token {
                kind: TokenType::RightBracket,
                line,
                column,
            },
            ',' => Token {
                kind: TokenType::Comma,
                line,
                column,
            },
            '.' => Token {
                kind: TokenType::Dot,
                line,
                column,
            },
            ';' => Token {
                kind: TokenType::Semicolon,
                line,
                column,
            },
            ':' => Token {
                kind: TokenType::Colon,
                line,
                column,
            },
            '=' => {
                if self.match_char('=') {
                    Token {
                        kind: TokenType::DoubleEqual,
                        line,
                        column,
                    }
                } else {
                    Token {
                        kind: TokenType::Equal,
                        line,
                        column,
                    }
                }
            }
            '+' => Token {
                kind: TokenType::Plus,
                line,
                column,
            },
            '-' if self.peek() == '>' => {
                self.advance();
                Token {
                    kind: TokenType::Arrow,
                    line,
                    column,
                }
            }
            '-' => Token {
                kind: TokenType::Minus,
                line,
                column,
            },
            '*' => Token {
                kind: TokenType::Star,
                line,
                column,
            },
            '/' => Token {
                kind: TokenType::Slash,
                line,
                column,
            },
            '%' => Token {
                kind: TokenType::Percent,
                line,
                column,
            },
            '!' => {
                if self.match_char('=') {
                    Token {
                        kind: TokenType::NotEqual,
                        line,
                        column,
                    }
                } else {
                    Token {
                        kind: TokenType::Not,
                        line,
                        column,
                    }
                }
            }
            '&' if self.match_char('&') => Token {
                kind: TokenType::And,
                line,
                column,
            },
            '|' if self.match_char('|') => Token {
                kind: TokenType::Or,
                line,
                column,
            },
            '-' if self.match_char('>') => Token {
                kind: TokenType::Arrow,
                line,
                column,
            },
            '<' => {
                if self.match_char('=') {
                    Token {
                        kind: TokenType::LessEqual,
                        line,
                        column,
                    }
                } else {
                    Token {
                        kind: TokenType::LessThan,
                        line,
                        column,
                    }
                }
            }
            '>' => {
                if self.match_char('=') {
                    Token {
                        kind: TokenType::GreaterEqual,
                        line,
                        column,
                    }
                } else {
                    Token {
                        kind: TokenType::GreaterThan,
                        line,
                        column,
                    }
                }
            }
            '"' => Token {
                kind: TokenType::StringLiteral(self.string()),
                line,
                column,
            },
            c if c.is_ascii_digit() => Token {
                kind: TokenType::IntegerLiteral(self.number()),
                line,
                column,
            },
            c if c.is_alphabetic() || c == '_' => {
                let ident = self.identifier();
                let kind = match ident.as_str() {
                    "fn" => TokenType::Fn,
                    "actor" => TokenType::Actor,
                    "let" => TokenType::Let,
                    "var" => TokenType::Var,
                    "return" => TokenType::Return,
                    "await" => TokenType::Await,
                    "for" => TokenType::For,
                    "while" => TokenType::While,
                    "in" => TokenType::In,
                    "break" => TokenType::Break,
                    "continue" => TokenType::Continue,
                    "if" => TokenType::If,
                    "else" => TokenType::Else,
                    "extern" => TokenType::Extern,
                    "unsafe" => TokenType::Unsafe,
                    "ui" => TokenType::Ui,
                    "child" => TokenType::Child,
                    "children" => TokenType::Children,
                    "i32" => TokenType::I32,
                    "str" => TokenType::Str,
                    "bool" => TokenType::Bool,
                    "ptr" => TokenType::Ptr,
                    "Option" => TokenType::Option,
                    "Result" => TokenType::Result,
                    _ => TokenType::Identifier(ident),
                };
                Token { kind, line, column }
            }
            _ => {
                // In MVP: report error or skip
                // Here a diagnostic could be recorded
                Token {
                    kind: TokenType::Identifier("INVALID".to_string()),
                    line,
                    column,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        let source = "fn main() { print(\"Hello\") }".to_string();
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        assert!(tokens.iter().any(|t| matches!(t.kind, TokenType::Fn)));
        assert!(tokens.iter().any(|t| matches!(t.kind, TokenType::StringLiteral(_))));
    }
}
