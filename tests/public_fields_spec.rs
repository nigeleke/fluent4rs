use fluent4rs::prelude::*;

#[test]
fn resource_entries_are_accesible() {
    let ftl0 = include_str!("full_grammar_example.ftl");
    let ast0 = Parser::parse(ftl0).unwrap();
    let entries = ast0.entries();
    assert!(!entries.is_empty());
}

#[test]
fn message_identifiers() {
    let ftl0 = include_str!("full_grammar_example.ftl");
    let ast0 = Parser::parse(ftl0).unwrap();
    let message_identifiers = ast0
        .entries()
        .iter()
        .filter_map(|e| match e {
            ResourceItem::Entry(Entry::Message(message)) => Some(message.identifier()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert!(!message_identifiers.is_empty());
}

#[test]
fn message_attributes() {
    let ftl0 = include_str!("full_grammar_example.ftl");
    let ast0 = Parser::parse(ftl0).unwrap();
    let message_attributes = ast0
        .entries()
        .iter()
        .filter_map(|e| match e {
            ResourceItem::Entry(Entry::Message(message)) => Some(message.attributes()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert!(!message_attributes.is_empty());
}

#[test]
fn term_identifiers() {
    let ftl0 = include_str!("full_grammar_example.ftl");
    let ast0 = Parser::parse(ftl0).unwrap();
    let term_identifiers = ast0
        .entries()
        .iter()
        .filter_map(|e| match e {
            ResourceItem::Entry(Entry::Term(term)) => Some(term.identifier()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert!(!term_identifiers.is_empty());
}

#[test]
fn term_patterns() {
    let ftl0 = include_str!("full_grammar_example.ftl");
    let ast0 = Parser::parse(ftl0).unwrap();
    let term_patterns = ast0
        .entries()
        .iter()
        .filter_map(|e| match e {
            ResourceItem::Entry(Entry::Term(term)) => Some(term.pattern()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert!(!term_patterns.is_empty());
}

#[test]
fn term_attributes() {
    let ftl0 = include_str!("full_grammar_example.ftl");
    let ast0 = Parser::parse(ftl0).unwrap();
    let term_attributes = ast0
        .entries()
        .iter()
        .filter_map(|e| match e {
            ResourceItem::Entry(Entry::Term(term)) => Some(term.attributes()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert!(!term_attributes.is_empty());
}
