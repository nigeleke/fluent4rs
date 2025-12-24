// #![feature(coverage_attribute)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![doc = include_str!("../README.md")]

pub mod ast;
mod grammar;
mod parser;
#[cfg(feature = "walker")]
mod walker;

/// The prelude module – a convenient way to import the most commonly used types and traits
/// when working with fluent4rs.
///
/// By importing `prelude::*`, users get immediate access to the core parsing API
/// (and optionally the AST walking utilities when the `walker` feature is enabled).
pub mod prelude {
    pub use crate::parser::{Parser, ParserError};

    #[cfg(feature = "walker")]
    pub use crate::walker::{Visitor, Walker};
}
