//! Core type definitions for the parsing system

/// Represents a CSS rule with its selector, properties, and nested rules
#[derive(Debug)]
pub struct Rule {
    /// The complete CSS selector for this rule
    pub selector: String,
    /// List of CSS properties for this rule
    pub properties: Vec<Property>,
    /// Nested rules (for handling nested selectors)
    pub children: Vec<Rule>,
}

/// Represents a single CSS property
#[derive(Debug)]
pub struct Property {
    /// The property name (e.g., "color", "margin")
    pub name: String,
    /// The property value (e.g., "#fff", "20px")
    pub value: String,
}