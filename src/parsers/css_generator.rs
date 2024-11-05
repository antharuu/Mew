//! CSS generation implementation
//!
//! Handles the final transformation of parsed rules into formatted CSS output.
//! The generator maintains proper indentation and spacing in the output CSS.

use super::traits::CssGenerator;
use super::types::Rule;

/// Implements CSS generation from parsed rules
pub struct MewCssGenerator;

impl MewCssGenerator {
    /// Creates a new instance of the CSS generator
    pub fn new() -> Self {
        Self
    }

    /// Recursively adds a rule and its children to the CSS output
    ///
    /// # Arguments
    /// * `css` - The string buffer to append CSS to
    /// * `rule` - The rule to process
    fn add_rule_to_css(&self, css: &mut String, rule: &Rule) {
        // Only add rules that have properties
        if !rule.properties.is_empty() {
            // Add the rule's selector and opening brace
            css.push_str(&format!("{} {{\n", rule.selector));

            // Add all properties with proper indentation
            for property in &rule.properties {
                css.push_str(&format!("    {}: {};\n", property.name, property.value));
            }

            // Close the rule block
            css.push_str("}\n\n");
        }

        // Process all nested rules recursively
        for child in &rule.children {
            self.add_rule_to_css(css, child);
        }
    }
}

impl CssGenerator for MewCssGenerator {
    fn generate(&self, rules: Vec<Rule>) -> String {
        let mut css = String::new();
        for rule in rules {
            self.add_rule_to_css(&mut css, &rule);
        }
        css
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::types::Property;
    use super::*;

    #[test]
    fn test_simple_css_generation() {
        let generator = MewCssGenerator::new();
        let rule = Rule {
            selector: ".button".to_string(),
            properties: vec![
                Property {
                    name: "color".to_string(),
                    value: "#ff0000".to_string(),
                }
            ],
            children: vec![],
        };

        let css = generator.generate(vec![rule]);
        assert!(css.contains(".button {"));
        assert!(css.contains("color: #ff0000;"));
        assert!(css.contains("}\n\n"));
    }

    #[test]
    fn test_nested_css_generation() {
        let generator = MewCssGenerator::new();
        let rule = Rule {
            selector: ".button".to_string(),
            properties: vec![
                Property {
                    name: "color".to_string(),
                    value: "#ff0000".to_string(),
                }
            ],
            children: vec![
                Rule {
                    selector: ".button__icon".to_string(),
                    properties: vec![
                        Property {
                            name: "size".to_string(),
                            value: "20px".to_string(),
                        }
                    ],
                    children: vec![],
                }
            ],
        };

        let css = generator.generate(vec![rule]);
        assert!(css.contains(".button__icon {"));
        assert!(css.contains("size: 20px;"));
    }
}