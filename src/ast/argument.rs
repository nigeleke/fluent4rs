use super::prelude::{InlineExpression, NamedArgument};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Argument {
    NamedArgument(NamedArgument),
    InlineExpression(InlineExpression),
}

impl std::fmt::Display for Argument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::NamedArgument(argument) => argument.to_string(),
            Self::InlineExpression(argument) => argument.to_string(),
        };
        write!(f, "{value}")
    }
}
