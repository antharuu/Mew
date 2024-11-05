// In selector_builder.rs

use super::traits::SelectorBuilder;

/// Implements BEM-style selector building
pub struct BemSelectorBuilder;

impl BemSelectorBuilder {
    /// Creates a new instance of the BEM selector builder
    pub fn new() -> Self {
        Self
    }
}

impl SelectorBuilder for BemSelectorBuilder {
    fn build_selector(&self, selector: &str, parent_selector: &str) -> String {
        // Return parent selector if current selector is empty
        if selector.trim().is_empty() {
            return parent_selector.to_string();
        }

        // If we have a parent selector
        if !parent_selector.is_empty() {
            let selector = selector.trim();

            // Handle pseudo-classes
            if selector.starts_with(':') {
                return format!("{}{}", parent_selector, selector);
            }

            // Handle element references with &
            if selector.starts_with('&') {
                let child_selector = selector.trim_start_matches('&').trim();
                if child_selector.starts_with(':') {
                    return format!("{}{}", parent_selector, child_selector);
                }
                return format!("{}__", parent_selector) + child_selector;
            }

            // Handle modifiers
            if selector.starts_with('@') {
                return format!("{}--{}", parent_selector, selector.trim_start_matches('@'));
            }

            // Handle nested elements
            return format!("{}__", parent_selector) + selector;
        }

        // Handle root selectors
        if !selector.starts_with('.') {
            return format!(".{}", selector.trim());
        }

        selector.trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bem_selector_basic() {
        let builder = BemSelectorBuilder::new();
        assert_eq!(builder.build_selector("button", ""), ".button");
    }

    #[test]
    fn test_bem_selector_with_dot() {
        let builder = BemSelectorBuilder::new();
        assert_eq!(builder.build_selector(".button", ""), ".button");
    }

    #[test]
    fn test_bem_selector_element() {
        let builder = BemSelectorBuilder::new();
        assert_eq!(builder.build_selector("&icon", ".button"), ".button__icon");
    }

    #[test]
    fn test_bem_selector_modifier() {
        let builder = BemSelectorBuilder::new();
        assert_eq!(builder.build_selector("@primary", ".button"), ".button--primary");
    }

    #[test]
    fn test_bem_selector_nested() {
        let builder = BemSelectorBuilder::new();
        assert_eq!(builder.build_selector("text", ".button__icon"), ".button__icon__text");
    }

    #[test]
    fn test_bem_selector_pseudo() {
        let builder = BemSelectorBuilder::new();
        assert_eq!(builder.build_selector("&:hover", ".button"), ".button:hover");
    }
}