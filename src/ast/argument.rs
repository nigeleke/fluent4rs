use super::{InlineExpression, NamedArgument};

#[cfg(feature = "walker")]
use crate::walker::{Visitor, Walkable};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [Argument](crate::ast::Argument) ::= [NamedArgument](crate::ast::NamedArgument)
///    | [InlineExpression](crate::ast::InlineExpression)
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Argument {
    NamedArgument(NamedArgument),
    InlineExpression(InlineExpression),
}

#[cfg(feature = "walker")]
impl Walkable for Argument {
    fn walk(&self, visitor: &mut dyn Visitor) {
        visitor.visit_argument(self);
        match self {
            Self::NamedArgument(argument) => argument.walk(visitor),
            Self::InlineExpression(expression) => expression.walk(visitor),
        }
    }
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
