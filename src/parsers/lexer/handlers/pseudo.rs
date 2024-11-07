// Pseudo handler
//
// Handles pseudo-classes like ':hover', ':active', etc.
use crate::parsers::lexer::{
    cursor::Cursor,
    token::{Token, TokenType, Span},
    error::{LexerError, Result},
};

pub struct PseudoHandler;

impl PseudoHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn handle(&mut self, cursor: &mut Cursor) -> Result<Token> {
        let start = cursor.position();
        // The ':' has already been consumed
        let pseudo = cursor.eat_while(|c| c.is_alphanumeric() || c == '-');
        if pseudo.is_empty() {
            return Err(LexerError::SyntaxError {
                message: "Expected pseudo-class name after ':'".to_string(),
                position: start,
            });
        }
        let end = cursor.position();
        Ok(Token::new(
            TokenType::PseudoClass(pseudo),
            Span::new(start, end),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::lexer::cursor::Cursor;

    #[test]
    fn test_valid_pseudo_class() {
        let mut cursor = Cursor::new("hover { color: blue; }");
        let mut handler = PseudoHandler::new();
        let token = handler.handle(&mut cursor).unwrap();
        match token.token_type {
            TokenType::PseudoClass(name) => {
                assert_eq!(name, "hover");
            }
            _ => panic!("Expected pseudo-class"),
        }
    }

    #[test]
    fn test_invalid_pseudo_class() {
        let mut cursor = Cursor::new(" { }");
        let mut handler = PseudoHandler::new();
        match handler.handle(&mut cursor) {
            Err(LexerError::SyntaxError { message, .. }) => {
                assert_eq!(message, "Expected pseudo-class name after ':'");
            }
            _ => panic!("Expected syntax error"),
        }
    }

    #[test]
    fn test_pseudo_class_with_dash() {
        let mut cursor = Cursor::new("first-child { }");
        let mut handler = PseudoHandler::new();
        let token = handler.handle(&mut cursor).unwrap();
        match token.token_type {
            TokenType::PseudoClass(name) => {
                assert_eq!(name, "first-child");
            }
            _ => panic!("Expected pseudo-class"),
        }
    }
}