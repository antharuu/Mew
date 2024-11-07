// Cursor
//
// Provides character-level cursor for lexing.
use std::str::Chars;
use std::iter::Peekable;
use super::token::Position;
use crate::parsers::lexer::error::{LexerError, Result}; // Import LexerError and Result

#[derive(Clone)]
pub struct Cursor<'source> {
    input: Peekable<Chars<'source>>,
    position: Position,
    source: &'source str,
}

impl<'source> Cursor<'source> {
    pub fn new(input: &'source str) -> Self {
        Self {
            input: input.chars().peekable(),
            position: Position::new(1, 1, 0),
            source: input,
        }
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    /// Peeks the second character ahead without consuming any input
    pub fn peek_second(&mut self) -> Option<char> {
        let mut iter = self.input.clone();
        iter.next(); // Skip the first character
        iter.next()
    }

    pub fn advance(&mut self) -> Option<char> {
        let c = self.input.next();
        if let Some(c) = c {
            self.position.advance(c);
        }
        c
    }

    pub fn eat_while<F>(&mut self, mut predicate: F) -> String
    where
        F: FnMut(char) -> bool,
    {
        let mut result = String::new();
        while let Some(&c) = self.peek() {
            if !predicate(c) {
                break;
            }
            result.push(self.advance().unwrap());
        }
        result
    }

    pub fn skip_whitespace(&mut self) {
        self.eat_while(char::is_whitespace);
    }

    pub fn is_eof(&mut self) -> bool {
        self.peek().is_none()
    }

    pub fn get_source(&self) -> &str {
        self.source
    }

    pub fn expect(&mut self, expected: char) -> Result<()> {
        match self.peek().copied() {
            Some(c) if c == expected => {
                self.advance();
                Ok(())
            }
            Some(c) => Err(LexerError::SyntaxError {
                message: format!("Expected '{}', found '{}'", expected, c),
                position: self.position(),
            }),
            None => Err(LexerError::SyntaxError {
                message: format!("Expected '{}', found end of file", expected),
                position: self.position(),
            }),
        }
    }

    pub fn read_value(&mut self) -> Result<String> {
        self.skip_whitespace();
        let value = self
            .eat_while(|c| c != ';' && c != '\n' && c != '}')
            .trim()
            .to_string();
        if value.is_empty() {
            return Err(LexerError::SyntaxError {
                message: "Expected value".to_string(),
                position: self.position(),
            });
        }
        // Consume ';' if present
        if let Some(';') = self.peek().copied() {
            self.advance();
        }
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_basic_operations() {
        let mut cursor = Cursor::new("abc");
        assert_eq!(cursor.peek(), Some(&'a'));
        assert_eq!(cursor.advance(), Some('a'));
        assert_eq!(cursor.advance(), Some('b'));
        assert_eq!(cursor.peek(), Some(&'c'));
    }

    #[test]
    fn test_cursor_eat_while() {
        let mut cursor = Cursor::new("aaabbb");
        let result = cursor.eat_while(|c| c == 'a');
        assert_eq!(result, "aaa");
        assert_eq!(cursor.peek(), Some(&'b'));
    }

    #[test]
    fn test_cursor_skip_whitespace() {
        let mut cursor = Cursor::new("  \t\n  abc");
        cursor.skip_whitespace();
        assert_eq!(cursor.peek(), Some(&'a'));
    }

    #[test]
    fn test_cursor_position_tracking() {
        let mut cursor = Cursor::new("a\nb");
        let start = cursor.position();
        cursor.advance(); // 'a'
        cursor.advance(); // '\n'
        cursor.advance(); // 'b'

        assert_eq!(start.line, 1);
        assert_eq!(start.column, 1);
        assert_eq!(cursor.position().line, 2);
        assert_eq!(cursor.position().column, 2);
    }
}