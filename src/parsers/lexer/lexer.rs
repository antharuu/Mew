// MewLexer
//
// The main lexer for the Mew language.
use super::{
    cursor::Cursor,
    error::{LexerError, Result},
    token::{Token, TokenType},
    traits::Lexer,
    handlers::{
        variable::VariableHandler,
        selector::SelectorHandler,
        property::PropertyHandler,
        bem::BemHandler,
        comment::CommentHandler,
        pseudo::PseudoHandler,
        brace::BraceHandler,
    },
};

/// The main lexer for the Mew language.
/// Coordinates different handlers for processing various token types.
pub struct MewLexer<'source> {
    cursor: Cursor<'source>,
    reached_eof: bool,
}

impl<'source> MewLexer<'source> {
    /// Creates a new MewLexer instance
    pub fn new(input: &'source str) -> Self {
        Self {
            cursor: Cursor::new(input),
            reached_eof: false,
        }
    }

    /// Creates an EOF token at the current position
    fn create_eof_token(&self) -> Result<Token> {
        let position = self.cursor.position();
        Ok(Token::new(
            TokenType::EOF,
            crate::parsers::lexer::token::Span::new(position, position),
        ))
    }

    /// Handles an unexpected character error
    fn handle_unexpected_char(&self, c: char) -> Result<Token> {
        Err(LexerError::UnexpectedChar {
            found: c,
            position: self.cursor.position(),
        })
    }
}

impl<'source> Lexer for MewLexer<'source> {
    fn next_token(&mut self) -> Result<Token> {
        if self.reached_eof {
            return self.create_eof_token();
        }
        self.cursor.skip_whitespace();
        if self.cursor.is_eof() {
            self.reached_eof = true;
            return self.create_eof_token();
        }
        // Peek at the next character to determine which handler to use
        match self.cursor.peek().copied() {
            None => {
                self.reached_eof = true;
                self.create_eof_token()
            }
            Some(c) => match c {
                // Variables ($variable)
                '$' => {
                    let mut handler = VariableHandler::new();
                    handler.handle(&mut self.cursor)
                }
                // Opening brace
                '{' => {
                    let mut handler = BraceHandler::new();
                    handler.handle_open(&mut self.cursor)
                }
                // Closing brace
                '}' => {
                    let mut handler = BraceHandler::new();
                    handler.handle_close(&mut self.cursor)
                }
                // Nested selectors and pseudo-classes (&:hover, &element)
                '&' => {
                    self.cursor.advance(); // Consume '&'
                    match self.cursor.peek().copied() {
                        Some(':') => {
                            let mut handler = PseudoHandler::new();
                            handler.handle(&mut self.cursor)
                        }
                        _ => {
                            let mut handler = SelectorHandler::new();
                            handler.handle_nested(&mut self.cursor)
                        }
                    }
                }
                // BEM modifiers (@modifier)
                '@' => {
                    let mut handler = BemHandler::new();
                    handler.handle(&mut self.cursor)
                }
                // Comments
                '/' if {
                    let mut temp_cursor = self.cursor.clone();
                    temp_cursor.advance(); // Skip '/'
                    matches!(temp_cursor.peek().copied(), Some('/'))
                } => {
                    let mut handler = CommentHandler::new();
                    handler.handle(&mut self.cursor)
                }
                // Properties or Selectors
                _ => {
                    let start_pos = self.cursor.position();
                    let identifier = self.cursor.eat_while(|c| {
                        c.is_alphanumeric() || c == '-' || c == '_'
                    });
                    if identifier.is_empty() {
                        self.handle_unexpected_char(c)
                    } else {
                        self.cursor.skip_whitespace();
                        match self.cursor.peek().copied() {
                            Some(':') => {
                                self.cursor.advance(); // Consume ':'
                                let mut handler = PropertyHandler::new();
                                handler.handle(&mut self.cursor, identifier, start_pos)
                            }
                            _ => {
                                let mut handler = SelectorHandler::new();
                                handler.handle_raw(&mut self.cursor, identifier, start_pos)
                            }
                        }
                    }
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        let mut lexer = MewLexer::new("");
        let token = lexer.next_token().unwrap();
        assert!(matches!(token.token_type, TokenType::EOF));
    }

    #[test]
    fn test_whitespace_only() {
        let mut lexer = MewLexer::new("   \t\n   ");
        let token = lexer.next_token().unwrap();
        assert!(matches!(token.token_type, TokenType::EOF));
    }

    #[test]
    fn test_basic_variable() {
        let mut lexer = MewLexer::new("$color: #fff;");
        let token = lexer.next_token().unwrap();
        match token.token_type {
            TokenType::VariableDeclaration(name, value) => {
                assert_eq!(name, "color");
                assert_eq!(value, "#fff");
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[test]
    fn test_basic_selector() {
        let mut lexer = MewLexer::new("button {");
        let token = lexer.next_token().unwrap();
        match token.token_type {
            TokenType::Selector(name) => {
                assert_eq!(name, "button");
            }
            _ => panic!("Expected selector"),
        }
    }

    #[test]
    fn test_basic_property() {
        let mut lexer = MewLexer::new("color: blue;");
        let token = lexer.next_token().unwrap();
        match token.token_type {
            TokenType::Property(name, value) => {
                assert_eq!(name, "color");
                assert_eq!(value, "blue");
            }
            _ => panic!("Expected property"),
        }
    }

    #[test]
    fn test_unexpected_character() {
        let mut lexer = MewLexer::new("~invalid");
        let result = lexer.next_token();
        assert!(matches!(result,
            Err(LexerError::UnexpectedChar { found: '~', .. })
        ));
    }

    #[test]
    fn test_comment() {
        let mut lexer = MewLexer::new("// This is a comment\nbutton {");
        let token = lexer.next_token().unwrap();
        match token.token_type {
            TokenType::Comment(content) => {
                assert_eq!(content, "This is a comment");
            }
            _ => panic!("Expected comment"),
        }
    }

    #[test]
    fn test_bem_modifier() {
        let mut lexer = MewLexer::new("@primary {");
        let token = lexer.next_token().unwrap();
        match token.token_type {
            TokenType::BemModifier(name) => {
                assert_eq!(name, "primary");
            }
            _ => panic!("Expected BEM modifier"),
        }
    }
}