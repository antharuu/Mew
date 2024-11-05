//! Rule parser implementation
//!
//! Handles parsing of rule blocks, including nested rules and properties.
//! The parser maintains the hierarchical structure of the rules while
//! processing properties and building selectors according to BEM methodology.

use std::collections::HashMap;
use super::traits::{RuleParser, PropertyParser, SelectorBuilder};
use super::types::Rule;

/// Implements rule parsing with support for property parsing and selector building
pub struct MewRuleParser<P: PropertyParser, S: SelectorBuilder> {
    property_parser: P,
    selector_builder: S,
}

impl<P: PropertyParser, S: SelectorBuilder> MewRuleParser<P, S> {
    /// Creates a new instance of the rule parser
    pub fn new(property_parser: P, selector_builder: S) -> Self {
        Self {
            property_parser,
            selector_builder,
        }
    }

    /// Extracts properties and nested rules from a rule body
    ///
    /// Returns a tuple of (properties_string, nested_rules_string)
    fn extract_properties_and_rules(&self, body: &str) -> (String, String) {
        let mut properties = Vec::new();
        let mut nested_rules = Vec::new();
        let mut current_block = String::new();
        let mut brace_count = 0;

        for line in body.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            if trimmed.contains('{') {
                brace_count += 1;
                current_block.push_str(line);
                current_block.push('\n');
            } else if trimmed.contains('}') {
                brace_count -= 1;
                current_block.push_str(line);
                current_block.push('\n');
                if brace_count == 0 {
                    nested_rules.push(current_block.clone());
                    current_block.clear();
                }
            } else if brace_count > 0 {
                current_block.push_str(line);
                current_block.push('\n');
            } else if trimmed.contains(':') {
                properties.push(trimmed);
            }
        }

        (properties.join(";\n"), nested_rules.join("\n"))
    }

    /// Processes a single rule string into a Rule structure
    fn process_rule(&self, rule_str: &str, parent_selector: &str, variables: &HashMap<String, String>) -> Option<Rule> {
        let parts: Vec<&str> = rule_str.splitn(2, '{').collect();
        if parts.len() != 2 {
            return None;
        }

        let selector = parts[0].trim();
        let body = parts[1].trim().trim_end_matches('}');

        if body.trim().is_empty() {
            return None;
        }

        let full_selector = self.selector_builder.build_selector(selector, parent_selector);
        let (properties, nested_rules) = self.extract_properties_and_rules(body);
        let processed_properties = self.property_parser.parse_properties(&properties, variables);
        let children = self.parse_rules(&nested_rules, &full_selector, variables);

        Some(Rule {
            selector: full_selector,
            properties: processed_properties,
            children,
        })
    }
}

impl<P: PropertyParser, S: SelectorBuilder> RuleParser for MewRuleParser<P, S> {
    fn parse_rules(&self, content: &str, parent_selector: &str, variables: &HashMap<String, String>) -> Vec<Rule> {
        let mut rules = Vec::new();
        let mut current_rule = String::new();
        let mut brace_count = 0;
        let mut in_rule = false;

        // Process character by character to properly handle nested rules
        for c in content.chars() {
            match c {
                '{' => {
                    brace_count += 1;
                    if brace_count == 1 {
                        in_rule = true;
                    }
                    current_rule.push(c);
                }
                '}' => {
                    brace_count -= 1;
                    current_rule.push(c);
                    if brace_count == 0 && in_rule {
                        if let Some(rule) = self.process_rule(&current_rule, parent_selector, variables) {
                            rules.push(rule);
                        }
                        current_rule.clear();
                        in_rule = false;
                    }
                }
                _ => {
                    if in_rule || (!in_rule && !c.is_whitespace()) {
                        current_rule.push(c);
                    }
                }
            }
        }

        rules
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::property_parser::MewPropertyParser;
    use crate::parsers::selector_builder::BemSelectorBuilder;

    #[test]
    fn test_simple_rule_parsing() {
        let parser = MewRuleParser::new(
            MewPropertyParser::new(),
            BemSelectorBuilder::new(),
        );
        let content = ".button { color: red; }";
        let variables = HashMap::new();

        let rules = parser.parse_rules(content, "", &variables);
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].selector, ".button");
        assert_eq!(rules[0].properties[0].name, "color");
        assert_eq!(rules[0].properties[0].value, "red");
    }

    #[test]
    fn test_nested_rule_parsing() {
        let parser = MewRuleParser::new(
            MewPropertyParser::new(),
            BemSelectorBuilder::new(),
        );
        let content = "
            .button {
                color: red;
                &icon {
                    size: 20px;
                }
            }
        ";
        let variables = HashMap::new();

        let rules = parser.parse_rules(content, "", &variables);
        assert_eq!(rules[0].selector, ".button");
        assert_eq!(rules[0].children.len(), 1);
        assert_eq!(rules[0].children[0].selector, ".button__icon");
        assert_eq!(rules[0].children[0].properties[0].name, "size");
        assert_eq!(rules[0].children[0].properties[0].value, "20px");
    }

    #[test]
    fn test_selector_without_dot() {
        let parser = MewRuleParser::new(
            MewPropertyParser::new(),
            BemSelectorBuilder::new(),
        );
        let content = "button { color: red; }";
        let variables = HashMap::new();

        let rules = parser.parse_rules(content, "", &variables);
        assert_eq!(rules[0].selector, ".button");
    }

    #[test]
    fn test_nested_modifier() {
        let parser = MewRuleParser::new(
            MewPropertyParser::new(),
            BemSelectorBuilder::new(),
        );
        let content = "
            .button {
                @primary {
                    background: blue;
                }
            }
        ";
        let variables = HashMap::new();

        let rules = parser.parse_rules(content, "", &variables);
        assert_eq!(rules[0].children[0].selector, ".button--primary");
    }
}