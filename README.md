# fluent4rs

A Fluent (language translation) resource file codec.

## Background

The [fluent-syntax](https://crates.io/crates/fluent_syntax) crate from [Project Fluent](https://projectfluent.org/)
provides a one way (file/string to AST) conversion of [Fluent FTL files](https://projectfluent.org/fluent/guide/).

This crate enables conversion in both directions.

It has been written for [lingora](https://github.com/nigeleke/lingora) (A localization management program), and may be
found to be useful outside of that context.

It is not intended to replace the any aspects [fluent-rs](https://github.com/projectfluent/fluent-rs)
crate implemented by [Project Fluent](https://projectfluent.org/), and, more the majority of language
translation needs, the reader is referred back to that crate.

## Development

```bash
cargo test
cargo build
```
