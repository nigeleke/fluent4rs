use super::prelude::{InlineExpression, SelectExpression};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum InlinePlaceable {
    SelectExpression(SelectExpression),
    InlineExpression(InlineExpression),
}

impl std::fmt::Display for InlinePlaceable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::SelectExpression(expression) => expression.to_string(),
            Self::InlineExpression(expression) => expression.to_string(),
        };
        write!(f, "{{ {value} }}")
    }
}
