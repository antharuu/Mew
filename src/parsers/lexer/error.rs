use super::token::Position;
use std::fmt;
use std::error::Error;

pub type Result<T> = std::result::Result<T, LexerError>;

#[derive(Debug)]
pub enum LexerError {
    SyntaxError { message: String, position: Position },
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SyntaxError { message, position } => write!(
                f,
                "{} at line {}, column {}",
                message, position.line, position.column
            ),
        }
    }
}

impl Error for LexerError {}