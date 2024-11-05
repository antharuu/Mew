//! Variable parser implementation
//!
//! Handles the parsing and substitution of Mew variables.
//! Variables are declared using the format: $variable-name: value;

use std::collections::HashMap;
use regex::Regex;
use super::traits::VariableParser;

/// Implements variable parsing for Mew syntax
pub struct MewVariableParser;

impl MewVariableParser {
    /// Creates a new instance of the variable parser
    pub fn new() -> Self {
        Self
    }
}

impl VariableParser for MewVariableParser {
    fn parse_variables(&self, content: &str) -> HashMap<String, String> {
        let mut variables = HashMap::new();
        // Matches variable declarations in the format: $name: value;
        let var_regex = Regex::new(r"\$([a-zA-Z-]+)\s*:\s*([^;]+);").unwrap();

        for cap in var_regex.captures_iter(content) {
            let name = format!("${}", &cap[1]);
            let value = cap[2].trim().to_string();
            variables.insert(name, value);
        }

        variables
    }

    fn remove_variables(&self, content: &str) -> String {
        let var_regex = Regex::new(r"\$[a-zA-Z-]+\s*:\s*[^;]+;").unwrap();
        var_regex.replace_all(content, "").into_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_parsing() {
        let parser = MewVariableParser::new();
        let content = r#"
            $primary-color: #ff0000;
            $spacing: 20px;
            .header {
                color: $primary-color;
                margin: $spacing;
            }
        "#;

        let variables = parser.parse_variables(content);
        assert_eq!(variables.get("$primary-color").unwrap(), "#ff0000");
        assert_eq!(variables.get("$spacing").unwrap(), "20px");
    }

    #[test]
    fn test_variable_removal() {
        let parser = MewVariableParser::new();
        let content = r#"
            $primary-color: #ff0000;
            .header {
                color: $primary-color;
            }
        "#;

        let cleaned = parser.remove_variables(content);
        assert!(!cleaned.contains("$primary-color: #ff0000;"));
        assert!(cleaned.contains(".header"));
        assert!(cleaned.contains("color: $primary-color;"));
    }
}