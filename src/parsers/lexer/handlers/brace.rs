// Brace handler
//
// Handles opening and closing braces.
use crate::parsers::lexer::{
    cursor::Cursor,
    token::{Token, TokenType, Span},
    error::Result,
};

pub struct BraceHandler;

impl BraceHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn handle_open(&mut self, cursor: &mut Cursor) -> Result<Token> {
        let start = cursor.position();
        cursor.advance(); // Skip '{'
        let end = cursor.position();
        Ok(Token::new(
            TokenType::BraceOpen,
            Span::new(start, end),
        ))
    }

    pub fn handle_close(&mut self, cursor: &mut Cursor) -> Result<Token> {
        let start = cursor.position();
        cursor.advance(); // Skip '}'
        let end = cursor.position();
        Ok(Token::new(
            TokenType::BraceClose,
            Span::new(start, end),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::lexer::cursor::Cursor;

    #[test]
    fn test_open_brace() {
        let mut cursor = Cursor::new("{");
        let mut handler = BraceHandler::new();
        let token = handler.handle_open(&mut cursor).unwrap();
        assert!(matches!(token.token_type, TokenType::BraceOpen));
    }

    #[test]
    fn test_close_brace() {
        let mut cursor = Cursor::new("}");
        let mut handler = BraceHandler::new();
        let token = handler.handle_close(&mut cursor).unwrap();
        assert!(matches!(token.token_type, TokenType::BraceClose));
    }
}
