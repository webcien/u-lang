// diagnostics.rs — U v0.8 Diagnostic System
// MIT License — Copyright (c) 2025 Webcien and U contributors
//
// Enhanced error reporting and diagnostics
// Features:
// - Detailed error messages with line/column information
// - Error suggestions and hints
// - Warning system
// - Error context display
// - Color-coded output (when available)

use std::fmt;

/// Severity level for diagnostics
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Note,
    Warning,
    Error,
    Fatal,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Note => write!(f, "note"),
            Severity::Warning => write!(f, "warning"),
            Severity::Error => write!(f, "error"),
            Severity::Fatal => write!(f, "fatal error"),
        }
    }
}

/// Location in source code
#[derive(Debug, Clone, Copy)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl Location {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

/// Span in source code (start and end locations)
#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub start: Location,
    pub end: Location,
}

impl Span {
    pub fn new(start: Location, end: Location) -> Self {
        Self { start, end }
    }

    pub fn single(loc: Location) -> Self {
        Self {
            start: loc,
            end: loc,
        }
    }
}

/// Diagnostic message with context
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub severity: Severity,
    pub message: String,
    pub span: Option<Span>,
    pub suggestion: Option<String>,
    pub context: Option<String>,
}

impl Diagnostic {
    pub fn new(severity: Severity, message: String) -> Self {
        Self {
            severity,
            message,
            span: None,
            suggestion: None,
            context: None,
        }
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }

    pub fn with_suggestion(mut self, suggestion: String) -> Self {
        self.suggestion = Some(suggestion);
        self
    }

    pub fn with_context(mut self, context: String) -> Self {
        self.context = Some(context);
        self
    }

    pub fn error(message: String) -> Self {
        Self::new(Severity::Error, message)
    }

    pub fn warning(message: String) -> Self {
        Self::new(Severity::Warning, message)
    }

    pub fn note(message: String) -> Self {
        Self::new(Severity::Note, message)
    }

    pub fn fatal(message: String) -> Self {
        Self::new(Severity::Fatal, message)
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Main message
        if let Some(span) = self.span {
            write!(f, "{}: {}: {}", span.start, self.severity, self.message)?;
        } else {
            write!(f, "{}: {}", self.severity, self.message)?;
        }

        // Context
        if let Some(context) = &self.context {
            write!(f, "\n  {}", context)?;
        }

        // Suggestion
        if let Some(suggestion) = &self.suggestion {
            write!(f, "\n  suggestion: {}", suggestion)?;
        }

        Ok(())
    }
}

/// Diagnostic collector for gathering multiple diagnostics
pub struct DiagnosticCollector {
    diagnostics: Vec<Diagnostic>,
}

impl DiagnosticCollector {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    pub fn add(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub fn error(&mut self, message: String) {
        self.add(Diagnostic::error(message));
    }

    pub fn warning(&mut self, message: String) {
        self.add(Diagnostic::warning(message));
    }

    pub fn note(&mut self, message: String) {
        self.add(Diagnostic::note(message));
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.severity >= Severity::Error)
    }

    pub fn has_warnings(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.severity == Severity::Warning)
    }

    pub fn error_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|d| d.severity >= Severity::Error)
            .count()
    }

    pub fn warning_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Warning)
            .count()
    }

    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    pub fn print_all(&self) {
        for diag in &self.diagnostics {
            println!("{}", diag);
        }
        if self.has_errors() {
            println!(
                "\nerror: {} error(s), {} warning(s)",
                self.error_count(),
                self.warning_count()
            );
        }
    }

    pub fn clear(&mut self) {
        self.diagnostics.clear();
    }
}

/// Common error messages with suggestions
pub mod errors {
    use super::*;

    pub fn undefined_variable(name: &str) -> Diagnostic {
        Diagnostic::error(format!("undefined variable '{}'", name))
            .with_suggestion(format!("did you mean to declare '{}'?", name))
    }

    pub fn type_mismatch(expected: &str, found: &str) -> Diagnostic {
        Diagnostic::error(format!("type mismatch: expected '{}', found '{}'", expected, found))
            .with_suggestion(format!("convert '{}' to '{}'", found, expected))
    }

    pub fn cannot_assign_immutable(name: &str) -> Diagnostic {
        Diagnostic::error(format!("cannot assign to immutable variable '{}'", name))
            .with_suggestion(format!("consider using 'var {}' instead of 'let {}'", name, name))
    }

    pub fn duplicate_definition(name: &str) -> Diagnostic {
        Diagnostic::error(format!("duplicate definition of '{}'", name))
            .with_suggestion(format!("rename one of the definitions"))
    }

    pub fn function_not_found(name: &str) -> Diagnostic {
        Diagnostic::error(format!("function '{}' not found", name))
            .with_suggestion(format!("check the spelling of '{}'", name))
    }

    pub fn wrong_argument_count(name: &str, expected: usize, found: usize) -> Diagnostic {
        Diagnostic::error(format!(
            "function '{}' expects {} arguments, found {}",
            name, expected, found
        ))
        .with_suggestion(format!(
            "adjust the number of arguments to {}",
            expected
        ))
    }

    pub fn ownership_violation(name: &str) -> Diagnostic {
        Diagnostic::error(format!(
            "value '{}' has been moved and is no longer available",
            name
        ))
        .with_suggestion(format!("use '{}' before it was moved", name))
    }

    pub fn data_race_detected() -> Diagnostic {
        Diagnostic::error("potential data race detected".to_string())
            .with_suggestion("use message passing instead of shared memory".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagnostic_creation() {
        let diag = Diagnostic::error("test error".to_string());
        assert_eq!(diag.severity, Severity::Error);
        assert_eq!(diag.message, "test error");
    }

    #[test]
    fn test_diagnostic_with_span() {
        let span = Span::new(Location::new(1, 5), Location::new(1, 10));
        let diag = Diagnostic::error("test error".to_string()).with_span(span);
        assert!(diag.span.is_some());
    }

    #[test]
    fn test_diagnostic_collector() {
        let mut collector = DiagnosticCollector::new();
        collector.error("error 1".to_string());
        collector.warning("warning 1".to_string());
        assert_eq!(collector.error_count(), 1);
        assert_eq!(collector.warning_count(), 1);
        assert!(collector.has_errors());
        assert!(collector.has_warnings());
    }

    #[test]
    fn test_error_messages() {
        let diag = errors::undefined_variable("x");
        assert!(diag.message.contains("undefined variable"));
        assert!(diag.suggestion.is_some());
    }
}
