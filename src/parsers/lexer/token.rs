//! Token definitions for the Mew preprocessor
//!
//! This module contains the token types and related structures used
//! for lexical analysis of Mew source files.

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    VariableDeclaration(String, String),  // ($name, value)
    Selector(String),                     // CSS selector
    BraceOpen,
    BraceClose,
    Property(String, String),             // (name, value)
    PseudoClass(String),                  // :hover, :active, etc.
    BemModifier(String),                  // @modifier name
    Comment(String),                      // Comment content
    EOF,
}

impl TokenType {
    /// Returns a human-readable description of the token type
    pub fn description(&self) -> String {
        match self {
            Self::VariableDeclaration(name, _) => format!("variable declaration '{}'", name),
            Self::Selector(s) => format!("selector '{}'", s),
            Self::BraceOpen => "opening brace '{'".to_string(),
            Self::BraceClose => "closing brace '}'".to_string(),
            Self::Property(name, _) => format!("property '{}'", name),
            Self::PseudoClass(p) => format!("pseudo-class ':'{}", p),
            Self::BemModifier(m) => format!("BEM modifier '@{}'", m),
            Self::Comment(_) => "comment".to_string(),
            Self::EOF => "end of file".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub span: Span,
}

impl Token {
    /// Creates a new token with the given type and span
    pub fn new(token_type: TokenType, span: Span) -> Self {
        Self { token_type, span }
    }

    /// Returns the human-readable location of this token
    pub fn location(&self) -> String {
        format!("line {}, column {}", self.span.start.line, self.span.start.column)
    }

    /// Returns a formatted error message for this token
    pub fn error_message(&self, message: &str) -> String {
        format!("{} at {}", message, self.location())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Span {
    /// Creates a new span from start and end positions
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    /// Returns the length of this span in characters
    pub fn len(&self) -> usize {
        self.end.offset - self.start.offset
    }

    /// Returns whether this span is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

impl Position {
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Self { line, column, offset }
    }

    pub fn advance(&mut self, c: char) {
        self.offset += c.len_utf8();
        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_description() {
        let token_type = TokenType::VariableDeclaration("color".to_string(), "#fff".to_string());
        assert_eq!(token_type.description(), "variable declaration 'color'");
    }

    #[test]
    fn test_span_operations() {
        let start = Position::new(1, 1, 0);
        let end = Position::new(1, 5, 4);
        let span = Span::new(start, end);

        assert_eq!(span.len(), 4);
        assert!(!span.is_empty());
    }

    #[test]
    fn test_token_location() {
        let start = Position::new(1, 1, 0);
        let end = Position::new(1, 5, 4);
        let span = Span::new(start, end);
        let token = Token::new(TokenType::BraceOpen, span);

        assert_eq!(token.location(), "line 1, column 1");
    }

    #[test]
    fn test_token_error_message() {
        let start = Position::new(1, 1, 0);
        let end = Position::new(1, 5, 4);
        let span = Span::new(start, end);
        let token = Token::new(TokenType::BraceOpen, span);

        assert_eq!(
            token.error_message("Unexpected token"),
            "Unexpected token at line 1, column 1"
        );
    }
}