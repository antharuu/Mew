use crate::parsers::lexer::{
    error::{LexerError, Result},
    token::Position,
};

pub struct Validator;

impl Validator {
    pub fn validate_identifier(name: &str, position: Position) -> Result<()> {
        if name.is_empty() {
            return Err(LexerError::InvalidIdentifier {
                found: name.to_string(),
                position,
            });
        }

        // Premier caractère doit être une lettre ou underscore
        if let Some(first) = name.chars().next() {
            if !first.is_ascii_alphabetic() && first != '_' {
                return Err(LexerError::InvalidIdentifier {
                    found: name.to_string(),
                    position,
                });
            }
        }

        // Valider tous les caractères
        for c in name.chars() {
            if !Self::is_valid_identifier_char(c) {
                return Err(LexerError::InvalidIdentifier {
                    found: name.to_string(),
                    position,
                });
            }
        }

        Ok(())
    }

    pub fn is_valid_identifier_char(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '-' || c == '_'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_identifier() {
        let pos = Position::new(1, 1, 0);
        assert!(Validator::validate_identifier("valid-name", pos).is_ok());
        assert!(Validator::validate_identifier("_private", pos).is_ok());
        assert!(Validator::validate_identifier("", pos).is_err());
        assert!(Validator::validate_identifier("123invalid", pos).is_err());
    }
}