use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use fluent4rs::{ast::Resource, prelude::Parser};

fn bench_parse(c: &mut Criterion) {
    let medium = black_box(include_str!("../tests/data/full_grammar_example.ftl"));

    let inputs = [("full_grammar_example", medium)];

    inputs
        .iter()
        .filter(|(_, input)| !input.is_empty())
        .for_each(|(name, input)| {
            c.bench_function(&format!("parse_{}", name), |b| {
                b.iter(|| {
                    let _ast: Resource = Parser::parse(black_box(input)).unwrap();
                    black_box(_ast);
                })
            });
        });
}

criterion_group!(benches, bench_parse);
criterion_main!(benches);
