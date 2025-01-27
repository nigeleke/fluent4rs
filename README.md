# fluent4rs

[![MIT License](https://img.shields.io/github/license/nigeleke/fluent4rs?style=plastic)](https://github.com/nigeleke/fluent4rs/blob/master/LICENSE)
[![Language](https://img.shields.io/badge/language-Rust-blue.svg?style=plastic)](https://www.rust-lang.org/)
[![Build](https://img.shields.io/github/actions/workflow/status/nigeleke/fluent4rs/acceptance.yml?style=plastic)](https://github.com/nigeleke/fluent4rs/actions/workflows/acceptance.yml)
[![Coverage](https://img.shields.io/codecov/c/github/nigeleke/fluent4rs?style=plastic)](https://codecov.io/gh/nigeleke/fluent4rs)
![Version](https://img.shields.io/github/v/tag/nigeleke/fluent4rs?style=plastic)

  [Site](https://nigeleke.github.io/fluent4rs) \| [GitHub](https://github.com/nigeleke/fluent4rs) \| [API](https://nigeleke.github.io/fluent4rs/api/fluent4rs/index.html) \| [Coverage Report](https://nigeleke.github.io/fluent4rs/coverage/index.html)

A Fluent (language translation) resource file codec.

## Background

The [fluent-syntax](https://crates.io/crates/fluent_syntax) crate from [Project Fluent](https://projectfluent.org/)
provides a one way (file/string to AST) conversion of [Fluent FTL files](https://projectfluent.org/fluent/guide/).

This crate enables conversion in both directions.

It has been written for [lingora](https://github.com/nigeleke/lingora) (A localization management program), and may be
found to be useful outside of that context. Only key AST fields are exposed, i.e. the minimal set required in the
[lingora](https://github.com/nigeleke/lingora) application. Feel free to lodge an issue to request additional fields.

It is not intended to replace any aspects of the [fluent-rs](https://github.com/projectfluent/fluent-rs)
crate implemented by [Project Fluent](https://projectfluent.org/), and, for the majority of language
translation needs, the reader is referred back to that crate.

## Development

```bash
cargo test
```
