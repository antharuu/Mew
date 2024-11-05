//! Property parser implementation
//!
//! Handles the parsing of CSS property declarations and variable substitution.
//! Properties are expected in the format: property-name: value;

use std::collections::HashMap;
use super::traits::PropertyParser;
use super::types::Property;

/// Implements property parsing and variable substitution for Mew syntax
pub struct MewPropertyParser;

impl MewPropertyParser {
    /// Creates a new instance of the property parser
    pub fn new() -> Self {
        Self
    }

    /// Checks if a string contains mathematical operators
    fn has_math_operations(&self, value: &str) -> bool {
        value.contains('+') || value.contains('-') ||
            value.contains('*') || value.contains('/')
    }

    /// Combines parts around an operator
    fn combine_math_expression(&self, parts: &[&str], start: usize) -> (String, usize) {
        let mut expr = Vec::new();
        let mut i = start;

        // Add the first part
        expr.push(parts[i].to_string());

        // Look ahead for operators and their values
        while i + 1 < parts.len() {
            let next = parts[i + 1];
            if next.starts_with('/') || next.starts_with('*') ||
                next.starts_with('+') || next.starts_with('-') ||
                expr.last().unwrap().ends_with('/') || expr.last().unwrap().ends_with('*') ||
                expr.last().unwrap().ends_with('+') || expr.last().unwrap().ends_with('-') {
                expr.push(next.to_string());
                i += 1;
            } else {
                break;
            }
        }

        (expr.join(""), i)
    }
}

impl PropertyParser for MewPropertyParser {
    fn parse_properties(&self, properties: &str, variables: &HashMap<String, String>) -> Vec<Property> {
        properties
            .split(';')
            .filter(|p| !p.trim().is_empty())
            .map(|p| {
                let parts: Vec<&str> = p.split(':').collect();
                if parts.len() == 2 {
                    let name = parts[0].trim().to_string();
                    let mut value = parts[1].trim().to_string();

                    // Replace variables first
                    for (var_name, var_value) in variables {
                        value = value.replace(var_name, var_value);
                    }

                    // If already wrapped in calc(), leave as is
                    if value.trim().starts_with("calc(") {
                        return Property { name, value };
                    }

                    // Process each part
                    let parts: Vec<&str> = value.split_whitespace().collect();
                    let mut new_parts = Vec::new();
                    let mut i = 0;

                    while i < parts.len() {
                        let current = parts[i];

                        // Check if we are starting a math expression
                        let needs_calc = self.has_math_operations(current) ||
                            (i + 1 < parts.len() && self.has_math_operations(parts[i + 1]));

                        if needs_calc {
                            let (expr, new_i) = self.combine_math_expression(&parts, i);
                            new_parts.push(format!("calc({})", expr));
                            i = new_i;
                        } else {
                            new_parts.push(current.to_string());
                        }

                        i += 1;
                    }

                    value = new_parts.join(" ");
                    Property { name, value }
                } else {
                    Property {
                        name: p.trim().to_string(),
                        value: String::new(),
                    }
                }
            })
            .filter(|p| !p.value.is_empty())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_property_parsing() {
        let parser = MewPropertyParser::new();
        let mut variables = HashMap::new();
        variables.insert("$color".to_string(), "#ff0000".to_string());

        let properties = "color: $color; padding: 10px;";
        let parsed = parser.parse_properties(properties, &variables);

        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].name, "color");
        assert_eq!(parsed[0].value, "#ff0000");
    }

    #[test]
    fn test_math_operations() {
        let parser = MewPropertyParser::new();
        let mut variables = HashMap::new();
        variables.insert("$spacing".to_string(), "16px".to_string());

        let properties = "padding: $spacing / 2; margin: 20px + 10px;";
        let parsed = parser.parse_properties(properties, &variables);

        assert_eq!(parsed[0].value, "calc(16px/2)");
        assert_eq!(parsed[1].value, "calc(20px+10px)");
    }

    #[test]
    fn test_complex_margin() {
        let parser = MewPropertyParser::new();
        let mut variables = HashMap::new();
        variables.insert("$card-spacing".to_string(), "20px".to_string());

        let properties = "margin: 0 0 $card-spacing / 4;";
        let parsed = parser.parse_properties(properties, &variables);

        assert_eq!(parsed[0].value, "0 0 calc(20px/4)");
    }

    #[test]
    fn test_nested_calc() {
        let parser = MewPropertyParser::new();
        let variables = HashMap::new();

        let properties = "width: calc(100% - 20px);";
        let parsed = parser.parse_properties(properties, &variables);

        assert_eq!(parsed[0].value, "calc(100% - 20px)");
    }

    #[test]
    fn test_variable_in_math() {
        let parser = MewPropertyParser::new();
        let mut variables = HashMap::new();
        variables.insert("$base".to_string(), "16px".to_string());

        let properties = "margin: $base * 2 + 8px;";
        let parsed = parser.parse_properties(properties, &variables);

        assert_eq!(parsed[0].value, "calc(16px*2+8px)");
    }

    #[test]
    fn test_mixed_values() {
        let parser = MewPropertyParser::new();
        let mut variables = HashMap::new();
        variables.insert("$gap".to_string(), "10px".to_string());

        let properties = "padding: 20px $gap / 2 15px;";
        let parsed = parser.parse_properties(properties, &variables);

        assert_eq!(parsed[0].value, "20px calc(10px/2) 15px");
    }
}