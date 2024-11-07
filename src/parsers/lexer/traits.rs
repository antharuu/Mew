use crate::parsers::lexer::{Token, Result, token::Span, token::Position};

pub trait Lexer {
    fn next_token(&mut self) -> Result<Token>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::lexer::TokenType;

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
            let end = Position::new(1, self.current, self.current);
            Ok(Token::new(token_type, Span::new(start, end)))
        }
    }

    #[test]
    fn test_mock_lexer() {
        let tokens = vec![
            TokenType::Selector("test".to_string()),
            TokenType::BraceOpen,
            TokenType::BraceClose,
        ];
        let mut lexer = MockLexer::new(tokens);

        let mut collected = Vec::new();
        while let Ok(token) = lexer.next_token() {
            if matches!(token.token_type, TokenType::EOF) {
                break;
            }
            collected.push(token);
        }
        assert_eq!(collected.len(), 3);
    }
}
