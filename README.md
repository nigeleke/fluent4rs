# fluent4rs

[![MIT License](https://img.shields.io/github/license/nigeleke/fluent4rs?style=plastic)](https://github.com/nigeleke/fluent4rs/blob/master/LICENSE)
[![Language](https://img.shields.io/badge/language-Rust-blue.svg?style=plastic)](https://www.rust-lang.org/)
[![Build](https://img.shields.io/github/actions/workflow/status/nigeleke/fluent4rs/acceptance.yml?style=plastic)](https://github.com/nigeleke/fluent4rs/actions/workflows/acceptance.yml)
[![Coverage](https://img.shields.io/codecov/c/github/nigeleke/fluent4rs?style=plastic)](https://codecov.io/gh/nigeleke/fluent4rs)
![Version](https://img.shields.io/github/v/tag/nigeleke/fluent4rs?style=plastic)

  [Site](https://nigeleke.github.io/fluent4rs) \| [GitHub](https://github.com/nigeleke/fluent4rs) \| [API](https://docs.rs/fluent4rs/latest/fluent4rs/) \| [Coverage Report](https://app.codecov.io/gh/nigeleke/fluent4rs)

A Fluent (language translation) resource file codec.

## Background

The [fluent-syntax](https://crates.io/crates/fluent_syntax) crate from [Project Fluent](https://projectfluent.org/)
parses [Fluent FTL files](https://projectfluent.org/fluent/guide/). It provides a deserialisation only from the
`Resource` level.

This crate:

* enables conversion in both directions for any node in the syntax tree,
* provides a syntax tree walker.

It has been written for [lingora](https://github.com/nigeleke/lingora) (a localization management program), and may be
found to be useful outside of that context.

It is not intended to replace any aspects of the [fluent-rs](https://github.com/projectfluent/fluent-rs)
crate implemented by [Project Fluent](https://projectfluent.org/), and, for the majority of language
translation needs, the reader is referred back to that crate.

| __Usage__                                        | [fluent4rs](https://nigeleke.github.io/fluent4rs/) | [fluent-syntax](https://crates.io/crates/fluent_syntax) |
| ------------------------------------------------ | -------------------------------------------------- | ------------------------------------------------------- |
| Programmatic inspection & editing of `ftl` files | ✓                                                  | ?                                                       |
| Language translation in a program                | x                                                  | [fluent](https://crates.io/crates/fluent)               |

## Features

| __Feature__  | __Description__                                                     |
|--------------|---------------------------------------------------------------------|
| default      | All features are disabled                                           |
| hash         | Allow AST nodes to be hashed, for potential usages in `HashMap`s    |
| serde        | Allow AST nodes to be serialised / deserialised                     |
| trace        | Include tracing to stderr in the DefaultVisitor implementation      |
| walker       | Provide AST walker and visitors                                     |

## Development

```bash
cargo test --all-features
```

## AST Image View

![Railroad](doc/fluent_ebnf.svg)

![Relationships](doc/fluent_relationships.svg)
