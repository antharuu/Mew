// Property handler
//
// Handles property declarations like `color: blue;`.
use crate::parsers::lexer::{
    cursor::Cursor,
    token::{Token, TokenType, Span, Position},
    error::Result, // Removed `LexerError` as it's unused
};

pub struct PropertyHandler;

impl PropertyHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn handle(
        &mut self,
        cursor: &mut Cursor,
        name: String,
        start_pos: Position,
    ) -> Result<Token> {
        // The ':' has already been consumed
        let value = cursor.read_value()?;
        let end = cursor.position();
        Ok(Token::new(
            TokenType::Property(name, value),
            Span::new(start_pos, end),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::lexer::cursor::Cursor;

    #[test]
    fn test_valid_property() {
        let mut cursor = Cursor::new(" blue;");
        let mut handler = PropertyHandler::new();
        let token = handler
            .handle(&mut cursor, "color".to_string(), Position::new(1, 1, 0))
            .unwrap();
        match token.token_type {
            TokenType::Property(name, value) => {
                assert_eq!(name, "color");
                assert_eq!(value, "blue");
            }
            _ => panic!("Expected property"),
        }
    }

    #[test]
    fn test_missing_value() {
        let mut cursor = Cursor::new(";");
        let mut handler = PropertyHandler::new();
        let result = handler.handle(&mut cursor, "color".to_string(), Position::new(1, 1, 0));
        assert!(result.is_err());
    }

    #[test]
    fn test_property_without_semicolon() {
        let mut cursor = Cursor::new(" blue");
        let mut handler = PropertyHandler::new();
        let token = handler
            .handle(&mut cursor, "color".to_string(), Position::new(1, 1, 0))
            .unwrap();
        match token.token_type {
            TokenType::Property(name, value) => {
                assert_eq!(name, "color");
                assert_eq!(value, "blue");
            }
            _ => panic!("Expected property"),
        }
    }
}