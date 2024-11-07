use super::token::Token;
use super::error::Result;

pub trait Lexer {
    fn next_token(&mut self) -> Result<Token>;
}