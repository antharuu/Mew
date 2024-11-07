// TokenReader
//
// Provides utility methods for reading tokens from the cursor.
use crate::parsers::lexer::{
    cursor::Cursor,
    error::{LexerError, Result},
    token::Position,
};

pub struct TokenReader<'cursor, 'source> {
    cursor: &'cursor mut Cursor<'source>,
}

impl<'cursor, 'source> TokenReader<'cursor, 'source>
where
    'source: 'cursor,
{
    pub fn new(cursor: &'cursor mut Cursor<'source>) -> Self {
        Self { cursor }
    }

    pub fn read_identifier(&mut self) -> String {
        self.cursor.eat_while(|c| Self::is_identifier_char(c))
    }

    pub fn read_value(&mut self) -> Result<String> {
        self.skip_whitespace();
        let value = self
            .cursor
            .eat_while(|c| c != '\n' && c != '}')
            .trim()
            .to_string();
        if value.is_empty() {
            return Err(LexerError::SyntaxError {
                message: "Expected value".to_string(),
                position: self.cursor.position(),
            });
        }
        Ok(value)
    }

    pub fn skip_terminators(&mut self) {
        self.skip_whitespace();
        if let Some(';') = self.cursor.peek().copied() {
            self.cursor.advance();
        }
        self.skip_whitespace();
    }

    pub fn position(&self) -> Position {
        self.cursor.position()
    }

    fn is_identifier_char(c: char) -> bool {
        c.is_alphanumeric() || c == '-' || c == '_'
    }

    pub fn skip_whitespace(&mut self) {
        self.cursor.eat_while(|c| c.is_whitespace());
    }

    pub fn skip_line(&mut self) {
        self.cursor.eat_while(|c| c != '\n');
        if let Some('\n') = self.cursor.peek().copied() {
            self.cursor.advance();
        }
    }

    pub fn expect(&mut self, expected: char) -> Result<()> {
        match self.cursor.peek().copied() {
            Some(c) if c == expected => {
                self.cursor.advance();
                Ok(())
            }
            Some(c) => Err(LexerError::SyntaxError {
                message: format!("Expected '{}', found '{}'", expected, c),
                position: self.cursor.position(),
            }),
            None => Err(LexerError::SyntaxError {
                message: format!("Expected '{}', found end of file", expected),
                position: self.cursor.position(),
            }),
        }
    }

    pub fn peek(&mut self) -> Option<char> {
        self.cursor.peek().copied()
    }

    pub fn advance(&mut self) -> Option<char> {
        self.cursor.advance()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_identifier() {
        let mut cursor = Cursor::new("my-variable123 rest");
        let mut reader = TokenReader::new(&mut cursor);
        assert_eq!(reader.read_identifier(), "my-variable123");
    }

    #[test]
    fn test_read_value() {
        let mut cursor = Cursor::new("  blue; next");
        let mut reader = TokenReader::new(&mut cursor);
        assert_eq!(reader.read_value().unwrap(), "blue");
    }

    #[test]
    fn test_skip_whitespace() {
        let mut cursor = Cursor::new("   abc");
        let mut reader = TokenReader::new(&mut cursor);
        reader.skip_whitespace();
        assert_eq!(reader.read_identifier(), "abc");
    }
}