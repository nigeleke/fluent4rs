#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![doc = include_str!("../README.md")]

pub mod ast;
mod error;
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
    #[cfg(feature = "walker")]
    pub use crate::walker::{Visitor, Walker};
    pub use crate::{error::Fluent4rsError, parser::Parser};
}
