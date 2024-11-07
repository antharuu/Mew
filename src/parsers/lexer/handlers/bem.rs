// BEM handler
//
// Handles BEM modifiers like `@modifier`.
use crate::parsers::lexer::{
    cursor::Cursor,
    token::{Token, TokenType, Span},
    error::{LexerError, Result},
};

pub struct BemHandler;

impl BemHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn handle(&mut self, cursor: &mut Cursor) -> Result<Token> {
        let start = cursor.position();
        cursor.advance(); // Skip '@'
        let modifier = cursor.eat_while(|c| c.is_alphanumeric() || c == '-' || c == '_');
        if modifier.is_empty() {
            return Err(LexerError::SyntaxError {
                message: "Expected modifier name after '@'".to_string(),
                position: start,
            });
        }
        let end = cursor.position();
        Ok(Token::new(
            TokenType::BemModifier(modifier),
            Span::new(start, end),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::lexer::cursor::Cursor;

    #[test]
    fn test_valid_modifier() {
        let mut cursor = Cursor::new("@primary { color: blue; }");
        let mut handler = BemHandler::new();
        let token = handler.handle(&mut cursor).unwrap();
        match token.token_type {
            TokenType::BemModifier(name) => {
                assert_eq!(name, "primary");
            }
            _ => panic!("Expected BEM modifier"),
        }
    }

    #[test]
    fn test_invalid_modifier() {
        let mut cursor = Cursor::new("@ { }");
        let mut handler = BemHandler::new();
        match handler.handle(&mut cursor) {
            Err(LexerError::SyntaxError { message, .. }) => {
                assert_eq!(message, "Expected modifier name after '@'");
            }
            _ => panic!("Expected syntax error"),
        }
    }
}
