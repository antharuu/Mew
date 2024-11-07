use crate::parsers::lexer::token::Position;
use std::fmt;
use std::error::Error;

pub type Result<T> = std::result::Result<T, LexerError>;

#[derive(Debug, PartialEq)]
pub enum LexerError {
    SyntaxError {
        message: String,
        position: Position,
    },
    UnexpectedChar {
        found: char,
        position: Position,
    },
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SyntaxError { message, position } => write!(
                f,
                "Syntax error: {} at line {}, column {}",
                message, position.line, position.column
            ),
            Self::UnexpectedChar { found, position } => write!(
                f,
                "Unexpected character '{}' at line {}, column {}",
                found, position.line, position.column
            ),
        }
    }
}

impl Error for LexerError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let pos = Position::new(1, 1, 0);

        let err = LexerError::SyntaxError {
            message: "Invalid token".to_string(),
            position: pos,
        };
        assert_eq!(
            err.to_string(),
            "Syntax error: Invalid token at line 1, column 1"
        );

        let err = LexerError::UnexpectedChar {
            found: '@',
            position: pos,
        };
        assert_eq!(
            err.to_string(),
            "Unexpected character '@' at line 1, column 1"
        );
    }
}