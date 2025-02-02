#![doc = include_str!("../README.md")]
mod ast;
mod grammar;
mod parser;
#[cfg(feature = "walker")]
mod walker;

pub mod prelude {
    pub use super::ast::*;
    pub use super::parser::{Parser, ParserError};
    #[cfg(feature = "walker")]
    pub use super::walker::{Visitor, Walker};
}
