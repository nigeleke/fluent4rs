#![doc = include_str!("../README.md")]
mod ast;
mod grammar;
mod parser;
#[cfg(feature = "walker")]
mod walker;

pub mod prelude {
    pub mod ast {
        pub use crate::ast::prelude::*;
    }

    pub mod parser {
        pub use crate::parser::{Parser, ParserError};
    }

    #[cfg(feature = "walker")]
    pub mod walker {
        pub use crate::walker::{Visitor, Walker};
    }
}
