//! Core parser module
//!
//! This module serves as an interface to the parsing subsystem,
//! coordinating the transformation of Mew syntax into CSS.

use crate::parsers::parse as parse_mew;

/// Parses Mew content and generates equivalent CSS
///
/// # Arguments
/// * `content` - The source Mew content to parse
///
/// # Returns
/// A String containing the generated CSS
pub fn parse(content: &str) -> String {
    parse_mew(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_parsing_pipeline() {
        let input = "
            $primary: #ff0000;
            .button {
                color: $primary;
                &icon {
                    size: 20px;
                }
                @primary {
                    background: $primary;
                }
            }
        ";

        let css = parse(input);
        assert!(css.contains(".button {"));
        assert!(css.contains("color: #ff0000;"));
        assert!(css.contains(".button__icon {"));
        assert!(css.contains(".button--primary {"));
    }

    #[test]
    fn test_empty_input() {
        let css = parse("");
        assert!(css.is_empty());
    }
}