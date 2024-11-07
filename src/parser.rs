//! Core parser module
//!
//! Currently only displays tokens for debugging purposes.

use crate::parsers::lexer::{MewLexer, Lexer};

/// Currently only displays tokens for debugging purposes.
///
/// # Arguments
/// * `content` - The source Mew content to tokenize
///
/// # Returns
/// A string containing the token stream as CSS comments
pub fn parse(content: &str) -> String {
    let mut lexer = MewLexer::new(content);
    let mut output = String::new();

    output.push_str("/* Tokens:\n");
    while let Ok(token) = lexer.next_token() {
        output.push_str(&format!(" * {:?}\n", token.token_type));
        if matches!(token.token_type, crate::parsers::lexer::TokenType::EOF) {
            break;
        }
    }
    output.push_str(" */\n");

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        let css = parse("");
        let expected = "/* Tokens:\n * EOF\n */\n";
        assert_eq!(css, expected);
    }

    #[test]
    fn test_full_parsing_pipeline() {
        let input = r#"
            $primary: #ff0000;
            button {
                color: $primary;
            }
        "#;
        let css = parse(input);
        println!("Generated tokens:\n{}", css);  // Debug output

        let expected = vec![
            "VariableDeclaration",
            "Selector",
            "BraceOpen",
            "Property",
            "BraceClose",
            "EOF"
        ];

        for token in expected {
            assert!(css.contains(token), "Missing token: {}", token);
        }
    }
}