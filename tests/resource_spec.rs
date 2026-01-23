use fluent4rs::ast::Resource;
use fluent4rs::prelude::Parser;

use pretty_assertions::assert_eq;

#[test]
fn resource_can_be_created_from_entries_only() {
    let ftl0 = include_str!("data/full_grammar_example.ftl");
    let ast0 = Parser::parse(ftl0).unwrap();

    let entries = Vec::from_iter(ast0.entries().into_iter().cloned());
    let ast1 = Resource::from(entries);

    assert_eq!(ast1.entries(), ast0.entries());
}
