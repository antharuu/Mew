//! Parser implementation modules
//!
//! This module hierarchy implements the core parsing functionality for Mew,
//! transforming Mew syntax into CSS through several processing stages:
//!
//! 1. Source preprocessing (comment removal, whitespace normalization)
//! 2. Variable parsing and substitution
//! 3. Property parsing and validation
//! 4. Selector building (BEM methodology)
//! 5. Rule parsing and nesting
//! 6. CSS generation

mod types;
mod traits;
mod preprocessor;
mod variable_parser;
mod property_parser;
mod selector_builder;
mod rule_parser;
mod css_generator;

use traits::*;
use preprocessor::SourcePreprocessor;
use variable_parser::MewVariableParser;
use property_parser::MewPropertyParser;
use selector_builder::BemSelectorBuilder;
use rule_parser::MewRuleParser;
use css_generator::MewCssGenerator;

/// Main parsing function that coordinates the transformation pipeline
pub fn parse(content: &str) -> String {
    // Initialize all parsers
    let preprocessor = SourcePreprocessor::new();
    let variable_parser = MewVariableParser::new();
    let property_parser = MewPropertyParser::new();
    let selector_builder = BemSelectorBuilder::new();
    let rule_parser = MewRuleParser::new(property_parser, selector_builder);
    let css_generator = MewCssGenerator::new();

    // Preprocess source content
    let clean_content = preprocessor.clean(content);

    // Parse variables and clean content
    let variables = variable_parser.parse_variables(&clean_content);
    let content_without_vars = variable_parser.remove_variables(&clean_content);

    // Parse rules with variable substitution
    let rules = rule_parser.parse_rules(&content_without_vars, "", &variables);

    // Generate final CSS
    css_generator.generate(rules)
}