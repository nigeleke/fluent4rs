use fluent4rs::prelude::Parser;

#[test]
fn full_grammar_can_be_parsed() {
    let ftl0 = include_str!("full_grammar_example.ftl");
    let ast0 = Parser::parse(ftl0);
    println!("AST0:: {:?}", ast0);
}
