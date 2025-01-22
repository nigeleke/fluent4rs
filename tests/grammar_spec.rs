use fluent4rs::prelude::Parser;

use pretty_assertions::assert_eq;

#[test]
fn full_grammar_can_be_parsed() {
    // ftl0 may not be layed out, but will be parsable.
    let ftl0 = include_str!("full_grammar_example.ftl");
    // let ftl0 = include_str!("one_off.ftl");
    let ast0 = Parser::parse(ftl0).unwrap();

    println!("======= AST0\n{:?}\n========\n", ast0);

    // ftl1 will laid out and will parse to same AST as ast0.
    let ftl1 = ast0.to_string();

    println!("********* FTL1\n{}\n********\n", ftl1);

    let ast1 = Parser::parse(&ftl1).unwrap();

    assert_eq!(ast1, ast0);

    // ,,, and layed out version of second ast should be the same as the first.
    let ftl2 = ast1.to_string();
    assert_eq!(ftl2, ftl1);
}
