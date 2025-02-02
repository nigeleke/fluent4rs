use super::{
    FunctionReference, InlinePlaceable, MessageReference, NumberLiteral, StringLiteral,
    TermReference, VariableReference,
};

#[cfg(feature = "walker")]
use crate::walker::{Visitor, Walkable};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum InlineExpression {
    StringLiteral(StringLiteral),
    NumberLiteral(NumberLiteral),
    FunctionReference(FunctionReference),
    MessageReference(MessageReference),
    TermReference(TermReference),
    VariableReference(VariableReference),
    InlinePlaceable(Box<InlinePlaceable>),
}

#[cfg(feature = "walker")]
impl Walkable for InlineExpression {
    fn walk(&self, visitor: &mut dyn Visitor) {
        visitor.visit_inline_expression(self);
        match self {
            Self::StringLiteral(literal) => literal.walk(visitor),
            Self::NumberLiteral(literal) => literal.walk(visitor),
            Self::FunctionReference(reference) => reference.walk(visitor),
            Self::MessageReference(reference) => reference.walk(visitor),
            Self::TermReference(reference) => reference.walk(visitor),
            Self::VariableReference(reference) => reference.walk(visitor),
            Self::InlinePlaceable(inline) => inline.walk(visitor),
        }
    }
}

impl std::fmt::Display for InlineExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::StringLiteral(literal) => literal.to_string(),
            Self::NumberLiteral(literal) => literal.to_string(),
            Self::FunctionReference(reference) => reference.to_string(),
            Self::MessageReference(reference) => reference.to_string(),
            Self::TermReference(reference) => reference.to_string(),
            Self::VariableReference(reference) => reference.to_string(),
            Self::InlinePlaceable(inline) => inline.to_string(),
        };
        write!(f, "{value}")
    }
}
