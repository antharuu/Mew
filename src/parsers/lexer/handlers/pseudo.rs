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

    /// Handles pseudo-class tokens after encountering ':'
    pub fn handle(&mut self, cursor: &mut Cursor) -> Result<Token> {
        let start = cursor.position();
        cursor.advance(); // Consume ':'

        // Read the pseudo-class name
        let pseudo = cursor.eat_while(|c| c.is_alphanumeric() || c == '-' || c == '_');
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
