use super::token::{Token, TokenType, Span};
use super::cursor::Cursor;
use super::error::{LexerError, Result};
use super::traits::Lexer;

pub struct MewLexer<'a> {
    cursor: Cursor<'a>,
    reached_eof: bool,
}

impl<'a> MewLexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            cursor: Cursor::new(input),
            reached_eof: false,
        }
    }

    fn read_identifier(&mut self) -> String {
        self.cursor.eat_while(|c| c.is_alphanumeric() || c == '-' || c == '_')
    }

    fn skip_semicolon(&mut self) {
        while let Some(&c) = self.cursor.peek() {
            if c == ';' || c == '\n' {
                self.cursor.advance();
            } else if !c.is_whitespace() {
                break;
            } else {
                self.cursor.advance();
            }
        }
    }

    fn read_value(&mut self) -> String {
        self.cursor.skip_whitespace();
        let value = self.cursor.eat_while(|c| c != ';' && c != '}' && c != '\n')
            .trim()
            .to_string();
        self.skip_semicolon();
        value
    }

    fn handle_property(&mut self, identifier: String) -> Result<Token> {
        let start = self.cursor.position();
        self.cursor.advance(); // Skip :
        let value = self.read_value();

        if value.is_empty() {
            return Err(LexerError::SyntaxError {
                message: "Expected property value after ':'".to_string(),
                position: self.cursor.position(),
            });
        }

        let end = self.cursor.position();
        Ok(Token {
            token_type: TokenType::Property(identifier, value),
            span: Span { start, end },
        })
    }

    fn handle_variable(&mut self) -> Result<Token> {
        let start = self.cursor.position();
        self.cursor.advance(); // Skip $

        let name = self.read_identifier();
        if name.is_empty() {
            return Err(LexerError::SyntaxError {
                message: "Expected identifier after '$'".to_string(),
                position: start,
            });
        }

        self.cursor.skip_whitespace();

        if self.cursor.peek() != Some(&':') {
            return Err(LexerError::SyntaxError {
                message: "Expected ':' after variable name".to_string(),
                position: self.cursor.position(),
            });
        }

        self.cursor.advance(); // Skip :
        let value = self.read_value();
        if value.is_empty() {
            return Err(LexerError::SyntaxError {
                message: "Expected value after ':'".to_string(),
                position: self.cursor.position(),
            });
        }

        let end = self.cursor.position();
        Ok(Token {
            token_type: TokenType::VariableDeclaration(name, value),
            span: Span { start, end },
        })
    }

    fn read_comment(&mut self) -> String {
        self.cursor.advance(); // Skip first /
        self.cursor.advance(); // Skip second /

        // Skip leading whitespace after //
        while let Some(&c) = self.cursor.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.cursor.advance();
        }

        // Read until newline and trim trailing whitespace and \r
        self.cursor.eat_while(|c| c != '\n')
            .trim_end()
            .to_string()
    }
}

impl<'a> Lexer for MewLexer<'a> {
    fn next_token(&mut self) -> Result<Token> {
        if self.reached_eof {
            return Ok(Token {
                token_type: TokenType::EOF,
                span: Span {
                    start: self.cursor.position(),
                    end: self.cursor.position(),
                },
            });
        }

        self.cursor.skip_whitespace();

        let start = self.cursor.position();

        match self.cursor.peek() {
            None => {
                self.reached_eof = true;
                Ok(Token {
                    token_type: TokenType::EOF,
                    span: Span {
                        start: self.cursor.position(),
                        end: self.cursor.position(),
                    },
                })
            }
            Some(&c) => match c {
                '$' => self.handle_variable(),
                '{' => {
                    self.cursor.advance();
                    Ok(Token {
                        token_type: TokenType::BraceOpen,
                        span: Span {
                            start,
                            end: self.cursor.position(),
                        },
                    })
                }
                '}' => {
                    self.cursor.advance();
                    Ok(Token {
                        token_type: TokenType::BraceClose,
                        span: Span {
                            start,
                            end: self.cursor.position(),
                        },
                    })
                }
                '&' => {
                    self.cursor.advance();
                    match self.cursor.peek() {
                        Some(&':') => {
                            self.cursor.advance();
                            let pseudo = self.read_identifier();
                            if pseudo.is_empty() {
                                return Err(LexerError::SyntaxError {
                                    message: "Expected pseudo-class name after ':'".to_string(),
                                    position: self.cursor.position(),
                                });
                            }
                            Ok(Token {
                                token_type: TokenType::PseudoClass(pseudo),
                                span: Span { start, end: self.cursor.position() },
                            })
                        }
                        _ => {
                            let element = self.read_identifier();
                            if element.is_empty() {
                                return Err(LexerError::SyntaxError {
                                    message: "Expected element name after '&'".to_string(),
                                    position: self.cursor.position(),
                                });
                            }
                            Ok(Token {
                                token_type: TokenType::Selector(format!("&{}", element)),
                                span: Span { start, end: self.cursor.position() },
                            })
                        }
                    }
                }
                '@' => {
                    self.cursor.advance();
                    let modifier = self.read_identifier();
                    if modifier.is_empty() {
                        return Err(LexerError::SyntaxError {
                            message: "Expected modifier name after '@'".to_string(),
                            position: self.cursor.position(),
                        });
                    }
                    Ok(Token {
                        token_type: TokenType::BemModifier(modifier),
                        span: Span { start, end: self.cursor.position() },
                    })
                }
                '/' if matches!(self.cursor.peek(), Some(&'/')) => {
                    let comment = self.read_comment();
                    Ok(Token {
                        token_type: TokenType::Comment(comment),
                        span: Span { start, end: self.cursor.position() },
                    })
                }
                _ => {
                    let identifier = self.read_identifier();
                    if identifier.is_empty() {
                        self.cursor.advance(); // Skip invalid character
                        return Err(LexerError::SyntaxError {
                            message: "Invalid character in input".to_string(),
                            position: self.cursor.position(),
                        });
                    }
                    self.cursor.skip_whitespace();

                    match self.cursor.peek() {
                        Some(&':') => self.handle_property(identifier),
                        _ => Ok(Token {
                            token_type: TokenType::Selector(identifier),
                            span: Span { start, end: self.cursor.position() },
                        }),
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
    fn test_comment_handling() {
        let mut lexer = MewLexer::new("// Simple comment\r\n");
        let token = lexer.next_token().unwrap();
        match &token.token_type {
            TokenType::Comment(content) => {
                assert_eq!(content, "Simple comment");
            }
            _ => panic!("Expected Comment token"),
        }
    }

    #[test]
    fn test_comment_with_leading_spaces() {
        let mut lexer = MewLexer::new("//    Indented comment   \r\n");
        let token = lexer.next_token().unwrap();
        match &token.token_type {
            TokenType::Comment(content) => {
                assert_eq!(content, "Indented comment");
            }
            _ => panic!("Expected Comment token"),
        }
    }

    #[test]
    fn test_variable_declaration() {
        let mut lexer = MewLexer::new("$button-color: #3498db;");
        let token = lexer.next_token().unwrap();
        match &token.token_type {
            TokenType::VariableDeclaration(name, value) => {
                assert_eq!(name, "button-color");
                assert_eq!(value, "#3498db");
            }
            _ => panic!("Expected VariableDeclaration"),
        }
    }

    #[test]
    fn test_invalid_variable_syntax() {
        let mut lexer = MewLexer::new("$: value;");
        match lexer.next_token() {
            Err(LexerError::SyntaxError { message, .. }) => {
                assert_eq!(message, "Expected identifier after '$'");
            }
            _ => panic!("Expected SyntaxError"),
        }
    }

    #[test]
    fn test_missing_variable_value() {
        let mut lexer = MewLexer::new("$name:;");
        match lexer.next_token() {
            Err(LexerError::SyntaxError { message, .. }) => {
                assert_eq!(message, "Expected value after ':'");
            }
            _ => panic!("Expected SyntaxError"),
        }
    }

    #[test]
    fn test_selector_and_properties() {
        let mut lexer = MewLexer::new("button { color: #fff; }");
        let mut tokens = Vec::new();

        while let Ok(token) = lexer.next_token() {
            if matches!(token.token_type, TokenType::EOF) {
                break;
            }
            tokens.push(token);
        }

        assert!(matches!(&tokens[0].token_type, TokenType::Selector(s) if s == "button"));
        assert!(matches!(tokens[1].token_type, TokenType::BraceOpen));
        assert!(matches!(&tokens[2].token_type, TokenType::Property(name, value)
            if name == "color" && value == "#fff"));
    }

    #[test]
    fn test_bem_modifier() {
        let mut lexer = MewLexer::new("@primary { background: blue; }");
        match &lexer.next_token().unwrap().token_type {
            TokenType::BemModifier(m) => assert_eq!(m, "primary"),
            _ => panic!("Expected BemModifier"),
        }
    }

    #[test]
    fn test_pseudo_class() {
        let mut lexer = MewLexer::new("&:hover { color: red; }");
        match &lexer.next_token().unwrap().token_type {
            TokenType::PseudoClass(p) => assert_eq!(p, "hover"),
            _ => panic!("Expected PseudoClass"),
        }
    }

    #[test]
    fn test_invalid_pseudo_class() {
        let mut lexer = MewLexer::new("&:");
        match lexer.next_token() {
            Err(LexerError::SyntaxError { message, .. }) => {
                assert_eq!(message, "Expected pseudo-class name after ':'");
            }
            _ => panic!("Expected SyntaxError"),
        }
    }

    #[test]
    fn test_invalid_bem_element() {
        let mut lexer = MewLexer::new("&");
        match lexer.next_token() {
            Err(LexerError::SyntaxError { message, .. }) => {
                assert_eq!(message, "Expected element name after '&'");
            }
            _ => panic!("Expected SyntaxError"),
        }
    }
}