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

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub span: Span,
}

impl Token {
    pub fn new(token_type: TokenType, span: Span) -> Self {
        Self { token_type, span }
    }

    /// Returns the location of the token for error reporting
    pub fn location(&self) -> String {
        format!(
            "line {}, column {}",
            self.span.start.line, self.span.start.column
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Span {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    pub fn len(&self) -> usize {
        self.end.offset - self.start.offset
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
    fn test_token_creation() {
        let start = Position::new(1, 1, 0);
        let end = Position::new(1, 4, 3);
        let span = Span::new(start, end);
        let token = Token::new(TokenType::Selector("div".to_string()), span);

        assert_eq!(token.location(), "line 1, column 1");
    }

    #[test]
    fn test_position_advance() {
        let mut pos = Position::new(1, 1, 0);
        pos.advance('a');
        assert_eq!(pos.column, 2);
        pos.advance('\n');
        assert_eq!(pos.line, 2);
        assert_eq!(pos.column, 1);
    }

    #[test]
    fn test_span_operations() {
        let start = Position::new(1, 1, 0);
        let end = Position::new(1, 4, 3);
        let span = Span::new(start, end);

        assert_eq!(span.len(), 3);
        assert!(!span.is_empty());
    }
}