//! Trait definitions for the parsing system components
//!
//! These traits define the core interfaces for the parsing system,
//! allowing for modular and extensible implementations.

use std::collections::HashMap;
use super::types::{Rule, Property};

/// Handles parsing and management of variables in Mew source files
pub trait VariableParser {
    /// Extracts variables and their values from the source content
    fn parse_variables(&self, content: &str) -> HashMap<String, String>;
    /// Removes variable declarations from the source content
    fn remove_variables(&self, content: &str) -> String;
}

/// Handles parsing of rules and their nested structure
pub trait RuleParser {
    /// Parses rules from the source content, handling nesting and variable substitution
    fn parse_rules(&self, content: &str, parent_selector: &str, variables: &HashMap<String, String>) -> Vec<Rule>;
}

/// Handles parsing of individual CSS properties
pub trait PropertyParser {
    /// Parses property declarations, including variable substitution
    fn parse_properties(&self, content: &str, variables: &HashMap<String, String>) -> Vec<Property>;
}

/// Handles building of CSS selectors according to the BEM methodology
pub trait SelectorBuilder {
    /// Builds a complete selector string based on current and parent selectors
    fn build_selector(&self, selector: &str, parent_selector: &str) -> String;
}

/// Handles generation of final CSS output
pub trait CssGenerator {
    /// Generates CSS string from parsed rules
    fn generate(&self, rules: Vec<Rule>) -> String;
}