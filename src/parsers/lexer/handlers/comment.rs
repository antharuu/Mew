// Comment handler
//
// Handles comments like `// This is a comment`.
use crate::parsers::lexer::{
    cursor::Cursor,
    token::{Token, TokenType, Span},
    error::Result,
};

pub struct CommentHandler;

impl CommentHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn handle(&mut self, cursor: &mut Cursor) -> Result<Token> {
        let start = cursor.position();
        cursor.advance(); // Skip first '/'
        cursor.advance(); // Skip second '/'

        // Skip leading whitespace after '//'
        cursor.skip_whitespace();

        let content = cursor
            .eat_while(|c| c != '\n')
            .trim_end()
            .trim_end_matches('\r')
            .to_string();

        // Consume the newline if present
        if let Some('\n') = cursor.peek().copied() {
            cursor.advance();
        }

        let end = cursor.position();
        Ok(Token::new(
            TokenType::Comment(content),
            Span::new(start, end),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::lexer::cursor::Cursor;

    #[test]
    fn test_basic_comment() {
        let mut cursor = Cursor::new("// This is a comment\n");
        let mut handler = CommentHandler::new();
        let token = handler.handle(&mut cursor).unwrap();
        match token.token_type {
            TokenType::Comment(content) => {
                assert_eq!(content, "This is a comment");
            }
            _ => panic!("Expected comment"),
        }
    }

    #[test]
    fn test_comment_without_newline() {
        let mut cursor = Cursor::new("// End of file comment");
        let mut handler = CommentHandler::new();
        let token = handler.handle(&mut cursor).unwrap();
        match token.token_type {
            TokenType::Comment(content) => {
                assert_eq!(content, "End of file comment");
            }
            _ => panic!("Expected comment"),
        }
    }

    #[test]
    fn test_empty_comment() {
        let mut cursor = Cursor::new("//\n");
        let mut handler = CommentHandler::new();
        let token = handler.handle(&mut cursor).unwrap();
        match token.token_type {
            TokenType::Comment(content) => {
                assert_eq!(content, "");
            }
            _ => panic!("Expected empty comment"),
        }
    }
}
