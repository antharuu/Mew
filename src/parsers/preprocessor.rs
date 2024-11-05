//! Source preprocessing module
//!
//! This module handles the preprocessing of Mew source code before parsing.
//! It is responsible for:
//! - Removing single-line and multi-line comments
//! - Normalizing whitespace and line endings
//! - Ensuring consistent source code formatting

use regex::Regex;

/// Preprocessor for Mew source code that handles cleaning and normalization
/// before the parsing phase.
pub struct SourcePreprocessor;

impl SourcePreprocessor {
    /// Creates a new instance of the source preprocessor
    pub fn new() -> Self {
        Self
    }

    /// Cleans and normalizes the source content
    pub fn clean(&self, content: &str) -> String {
        let content = self.remove_comments(content);
        self.normalize_whitespace(&content)
    }

    /// Removes all types of comments from the source code
    fn remove_comments(&self, content: &str) -> String {
        // Remove single-line comments (// ...)
        let single_line_comment = Regex::new(r"//[^\n]*").unwrap();
        let without_single_line = single_line_comment.replace_all(content, "");

        // Remove multi-line comments (/* ... */)
        let multi_line_comment = Regex::new(r"/\*[\s\S]*?\*/").unwrap();
        multi_line_comment.replace_all(&without_single_line, "").into_owned()
    }

    /// Normalizes whitespace in the source content
    fn normalize_whitespace(&self, content: &str) -> String {
        let empty_lines = Regex::new(r"\n\s*\n").unwrap();
        empty_lines.replace_all(content, "\n").into_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_single_line_comments() {
        let preprocessor = SourcePreprocessor::new();
        let content = r#"
            // This is a comment
            .button {
                color: red; // Inline comment
            }
        "#;

        let cleaned = preprocessor.clean(content);
        assert!(!cleaned.contains("This is a comment"));
        assert!(!cleaned.contains("Inline comment"));
        assert!(cleaned.contains(".button {"));
        assert!(cleaned.contains("color: red;"));
    }

    #[test]
    fn test_remove_multi_line_comments() {
        let preprocessor = SourcePreprocessor::new();
        let content = r#"
            /* This is a
               multi-line comment */
            .button {
                /* Another
                   comment */ color: red;
            }
        "#;

        let cleaned = preprocessor.clean(content);
        assert!(!cleaned.contains("This is a"));
        assert!(!cleaned.contains("multi-line comment"));
        assert!(cleaned.contains(".button {"));
        assert!(cleaned.contains("color: red;"));
    }

    #[test]
    fn test_preserve_content_structure() {
        let preprocessor = SourcePreprocessor::new();
        let content = r#"
            // Comment
            .button {
                /* Multi-line
                   comment */
                color: red;
                /* Comment */
                padding: 10px;
            }
        "#;

        let cleaned = preprocessor.clean(content);
        let lines: Vec<&str> = cleaned.lines().collect();
        assert!(lines.iter().any(|&line| line.trim() == ".button {"));
        assert!(lines.iter().any(|&line| line.trim() == "color: red;"));
        assert!(lines.iter().any(|&line| line.trim() == "padding: 10px;"));
    }
}