use super::ast::prelude::Resource;

use thiserror::*;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("failed to parse text; reason: {0}")]
    FailedToParse(String),
}

type Result<T> = std::result::Result<T, ParserError>;

pub struct Parser;

impl Parser {
    pub fn parse(text: &str) -> Result<Resource> {
        Self::parse_with_junk(text).and_then(junk_as_error)
    }

    pub fn parse_with_junk(text: &str) -> Result<Resource> {
        super::grammar::resource()
            .parse(text.as_bytes())
            .map_err(|e| ParserError::FailedToParse(e.to_string()))
    }
}

fn junk_as_error(resource: Resource) -> Result<Resource> {
    let junk = resource.junk();

    if junk.is_empty() {
        Ok(resource)
    } else {
        let with_errors = format!(
            "Invalid entries: {}",
            junk.iter()
                .map(|j| j.to_string())
                .collect::<Vec<_>>()
                .join("\n")
        );
        Err(ParserError::FailedToParse(with_errors))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ast::prelude::Entry;
    use std::collections::HashSet;

    #[test]
    fn resource_entries_are_accesible() {
        let ftl0 = include_str!("../tests/data/full_grammar_example.ftl");
        let ast0 = Parser::parse(ftl0).unwrap();
        let entries = ast0.entries();
        assert!(!entries.is_empty());
    }

    #[test]
    fn resource_junk_is_accesible() {
        let ftl0 = r#"sdfhkh &(*$%&$(*%&
$W&(*$&(*%&("#;
        let ast0 = Parser::parse_with_junk(ftl0).unwrap();
        let junk = ast0.junk();
        assert!(!junk.is_empty());
    }

    #[test]
    fn resource_junk_is_accesible_no_junk() {
        let ftl0 = include_str!("../tests/data/full_grammar_example.ftl");
        let ast0 = Parser::parse_with_junk(ftl0).unwrap();
        let junk = ast0.junk();
        assert!(junk.is_empty());
    }

    #[test]
    fn message_identifiers() {
        let ftl0 = include_str!("../tests/data/full_grammar_example.ftl");
        let ast0 = Parser::parse(ftl0).unwrap();
        let message_identifiers = ast0
            .entries()
            .iter()
            .filter_map(|e| match e {
                Entry::Message(message) => Some(message.identifier()),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert!(!message_identifiers.is_empty());
    }

    #[test]
    fn message_identifier_names() {
        let ftl0 = include_str!("../tests/data/identifier_names.ftl");
        let ast0 = Parser::parse(ftl0).unwrap();
        let identifiers = ast0
            .entries()
            .iter()
            .filter_map(|e| match e {
                Entry::Message(message) => Some(message.identifier()),
                Entry::Term(term) => Some(term.identifier()),
                _ => None,
            })
            .collect::<HashSet<_>>();
        assert_eq!(identifiers.len(), 1);

        let identifier_names = ast0
            .entries()
            .iter()
            .filter_map(|e| match e {
                Entry::Message(message) => Some(message.identifier_name()),
                Entry::Term(term) => Some(term.identifier_name()),
                _ => None,
            })
            .collect::<HashSet<_>>();
        assert_eq!(identifier_names.len(), 2);
    }

    #[test]
    fn message_attributes() {
        let ftl0 = include_str!("../tests/data/full_grammar_example.ftl");
        let ast0 = Parser::parse(ftl0).unwrap();
        let message_attributes = ast0
            .entries()
            .iter()
            .filter_map(|e| match e {
                Entry::Message(message) => Some(message.attributes()),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert!(!message_attributes.is_empty());
    }

    #[test]
    fn message_patterns() {
        let ftl0 = include_str!("../tests/data/full_grammar_example.ftl");
        let ast0 = Parser::parse(ftl0).unwrap();
        let message_patterns = ast0
            .entries()
            .iter()
            .filter_map(|e| match e {
                Entry::Message(message) => Some(message.pattern()),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert!(!message_patterns.is_empty());
    }

    #[test]
    fn term_identifiers() {
        let ftl0 = include_str!("../tests/data/full_grammar_example.ftl");
        let ast0 = Parser::parse(ftl0).unwrap();
        let term_identifiers = ast0
            .entries()
            .iter()
            .filter_map(|e| match e {
                Entry::Term(term) => Some(term.identifier()),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert!(!term_identifiers.is_empty());
    }

    #[test]
    fn term_patterns() {
        let ftl0 = include_str!("../tests/data/full_grammar_example.ftl");
        let ast0 = Parser::parse(ftl0).unwrap();
        let term_patterns = ast0
            .entries()
            .iter()
            .filter_map(|e| match e {
                Entry::Term(term) => Some(term.pattern()),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert!(!term_patterns.is_empty());
    }

    #[test]
    fn term_attributes() {
        let ftl0 = include_str!("../tests/data/full_grammar_example.ftl");
        let ast0 = Parser::parse(ftl0).unwrap();
        let term_attributes = ast0
            .entries()
            .iter()
            .filter_map(|e| match e {
                Entry::Term(term) => Some(term.attributes()),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert!(!term_attributes.is_empty());
    }
}
