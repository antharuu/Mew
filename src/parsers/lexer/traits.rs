use crate::parsers::lexer::{Token, Result};

pub trait Lexer {
    fn next_token(&mut self) -> Result<Token>;
}

pub trait LexerExt: Lexer {
    fn collect_tokens(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token()?;
            let is_eof = matches!(token.token_type, crate::parsers::lexer::TokenType::EOF);
            tokens.push(token);
            if is_eof {
                break;
            }
        }
        Ok(tokens)
    }
}

impl<T: Lexer> LexerExt for T {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::lexer::{TokenType, token::{Span, Position}};

    struct MockLexer {
        tokens: Vec<TokenType>,
        current: usize,
    }

    impl MockLexer {
        fn new(tokens: Vec<TokenType>) -> Self {
            Self {
                tokens,
                current: 0,
            }
        }
    }

    impl Lexer for MockLexer {
        fn next_token(&mut self) -> Result<Token> {
            if self.current >= self.tokens.len() {
                let pos = Position::new(1, 1, 0);
                return Ok(Token::new(TokenType::EOF, Span::new(pos, pos)));
            }

            let token_type = self.tokens[self.current].clone();
            self.current += 1;

            let start = Position::new(1, self.current, self.current - 1);
            let end = Position::new(1, self.current + 1, self.current);
            Ok(Token::new(token_type, Span::new(start, end)))
        }
    }

    #[test]
    fn test_collect_tokens() {
        let tokens = vec![
            TokenType::Selector("test".to_string()),
            TokenType::BraceOpen,
            TokenType::BraceClose,
        ];
        let mut lexer = MockLexer::new(tokens);

        let collected = lexer.collect_tokens().unwrap();
        assert_eq!(collected.len(), 4); // 3 tokens + EOF
        assert!(matches!(collected[3].token_type, TokenType::EOF));
    }
}