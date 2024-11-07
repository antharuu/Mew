// Variable handler
//
// Handles variable declarations like `$name: value;`.
use crate::parsers::lexer::{
    token::{Token, TokenType, Span},
    error::{LexerError, Result},
    cursor::Cursor,
};

pub struct VariableHandler;

impl VariableHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn handle(&mut self, cursor: &mut Cursor) -> Result<Token> {
        let start = cursor.position();
        cursor.advance(); // Skip '$'
        let name = cursor.eat_while(|c| c.is_alphanumeric() || c == '_' || c == '-');
        if name.is_empty() {
            return Err(LexerError::SyntaxError {
                message: "Expected identifier after '$'".to_string(),
                position: start,
            });
        }
        cursor.skip_whitespace();
        cursor.expect(':')?;
        let value = cursor.read_value()?;
        let end = cursor.position();
        Ok(Token::new(
            TokenType::VariableDeclaration(name, value),
            Span::new(start, end),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::lexer::cursor::Cursor;

    #[test]
    fn test_valid_variable() {
        let mut cursor = Cursor::new("$color: #fff;");
        let mut handler = VariableHandler::new();
        let token = handler.handle(&mut cursor).unwrap();
        match token.token_type {
            TokenType::VariableDeclaration(name, value) => {
                assert_eq!(name, "color");
                assert_eq!(value, "#fff");
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[test]
    fn test_missing_colon() {
        let mut cursor = Cursor::new("$color #fff;");
        let mut handler = VariableHandler::new();
        assert!(handler.handle(&mut cursor).is_err());
    }

    #[test]
    fn test_empty_identifier() {
        let mut cursor = Cursor::new("$: #fff;");
        let mut handler = VariableHandler::new();
        assert!(handler.handle(&mut cursor).is_err());
    }
}