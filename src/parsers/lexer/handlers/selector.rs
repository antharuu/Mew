// Selector handler
//
// Handles selectors like `button`, `.class`, and nested selectors like `&element`.
use crate::parsers::lexer::{
    cursor::Cursor,
    token::{Token, TokenType, Span, Position},
    error::{LexerError, Result},
};

pub struct SelectorHandler;

impl SelectorHandler {
    pub fn new() -> Self {
        Self
    }

    // For normal selectors (button, .class, etc.)
    pub fn handle_raw(
        &mut self,
        cursor: &mut Cursor,
        identifier: String,
        start_pos: Position,
    ) -> Result<Token> {
        let end = cursor.position();
        Ok(Token::new(
            TokenType::Selector(identifier),
            Span::new(start_pos, end),
        ))
    }

    // For nested selectors (&element)
    pub fn handle_nested(&mut self, cursor: &mut Cursor) -> Result<Token> {
        let start = cursor.position();
        let identifier = cursor.eat_while(|c| c.is_alphanumeric() || c == '-' || c == '_');
        if identifier.is_empty() {
            return Err(LexerError::SyntaxError {
                message: "Expected element name after '&'".to_string(),
                position: start,
            });
        }
        let end = cursor.position();
        Ok(Token::new(
            TokenType::Selector(format!("&{}", identifier)),
            Span::new(start, end),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::lexer::cursor::Cursor;

    #[test]
    fn test_basic_selector() {
        let mut cursor = Cursor::new("");
        let mut handler = SelectorHandler::new();
        let token = handler
            .handle_raw(&mut cursor, "button".to_string(), Position::new(1, 1, 0))
            .unwrap();
        match token.token_type {
            TokenType::Selector(name) => {
                assert_eq!(name, "button");
            }
            _ => panic!("Expected selector"),
        }
    }

    #[test]
    fn test_nested_selector() {
        let mut cursor = Cursor::new("header { }");
        let mut handler = SelectorHandler::new();
        let token = handler.handle_nested(&mut cursor).unwrap();
        match token.token_type {
            TokenType::Selector(name) => {
                assert_eq!(name, "&header");
            }
            _ => panic!("Expected nested selector"),
        }
    }

    #[test]
    fn test_invalid_nested_selector() {
        let mut cursor = Cursor::new(" { }");
        let mut handler = SelectorHandler::new();
        match handler.handle_nested(&mut cursor) {
            Err(LexerError::SyntaxError { message, .. }) => {
                assert_eq!(message, "Expected element name after '&'");
            }
            _ => panic!("Expected syntax error"),
        }
    }
}