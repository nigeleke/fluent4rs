#![doc = include_str!("../README.md")]
pub mod ast;
mod grammar;
pub mod parser;
#[cfg(feature = "walker")]
pub mod walker;

pub mod prelude {
    pub use crate::ast::prelude::*;
    pub use crate::parser::{Parser, ParserError};

    #[cfg(feature = "walker")]
    pub use crate::walker::{Visitor, Walker};
}
