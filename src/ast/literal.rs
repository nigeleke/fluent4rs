use super::prelude::{NumberLiteral, StringLiteral};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Literal {
    Number(NumberLiteral),
    String(StringLiteral),
}

impl From<NumberLiteral> for Literal {
    fn from(value: NumberLiteral) -> Self {
        Self::Number(value)
    }
}

impl From<StringLiteral> for Literal {
    fn from(value: StringLiteral) -> Self {
        Self::String(value)
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Number(literal) => literal.to_string(),
            Self::String(literal) => literal.to_string(),
        };
        write!(f, "{value}")
    }
}
