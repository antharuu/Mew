use std::str::Chars;
use std::iter::Peekable;
use super::token::Position;

pub struct Cursor<'a> {
    input: Peekable<Chars<'a>>,
    position: Position,
}

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
            position: Position::new(1, 1, 0),
        }
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    pub fn advance(&mut self) -> Option<char> {
        let c = self.input.next();
        if let Some(c) = c {
            self.position.advance(c);
        }
        c
    }

    pub fn eat_while<F>(&mut self, predicate: F) -> String
    where
        F: Fn(char) -> bool,
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
}