use super::{InlineExpression, SelectExpression};

#[cfg(feature = "walker")]
use crate::walker::{Visitor, Walkable, Walker};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// inline_placeable ::= "{" blank? ([SelectExpression](crate::ast::SelectExpression) | [InlineExpression](crate::ast::InlineExpression)) blank? "}"
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum InlinePlaceable {
    SelectExpression(SelectExpression),
    InlineExpression(InlineExpression),
}

#[cfg(feature = "walker")]
impl Walkable for InlinePlaceable {
    fn walk(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::SelectExpression(expression) => Walker::walk(expression, visitor),
            Self::InlineExpression(expression) => Walker::walk(expression, visitor),
        }
    }
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
