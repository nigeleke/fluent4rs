use super::{
    FunctionReference, InlinePlaceable, MessageReference, NumberLiteral, StringLiteral,
    TermReference, VariableReference,
};

#[cfg(feature = "walker")]
use crate::walker::{Visitor, Walkable, Walker};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [InlineExpression](crate::ast::InlineExpression) ::= [StringLiteral](crate::ast::StringLiteral)
///  | [NumberLiteral](crate::ast::NumberLiteral)
///  | [FunctionReference](crate::ast::FunctionReference)
///  | [MessageReference](crate::ast::MessageReference)
///  | [TermReference](crate::ast::TermReference)
///  | [VariableReference](crate::ast::VariableReference)
///  | inline_placeable
///
/// Rules for validating expressions in Placeables and as selectors of
/// SelectExpressions are documented in [spec/valid.md](https://github.com/projectfluent/fluent/blob/master/spec/valid.md) and enforced in
/// [syntax/abstract.js](https://github.com/projectfluent/fluent/blob/master/syntax/abstract.js).
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum InlineExpression {
    #[doc(hidden)]
    StringLiteral(StringLiteral),

    #[doc(hidden)]
    NumberLiteral(NumberLiteral),

    #[doc(hidden)]
    FunctionReference(FunctionReference),

    #[doc(hidden)]
    MessageReference(MessageReference),

    #[doc(hidden)]
    TermReference(TermReference),

    #[doc(hidden)]
    VariableReference(VariableReference),

    #[doc(hidden)]
    InlinePlaceable(Box<InlinePlaceable>),
}

#[cfg(feature = "walker")]
impl Walkable for InlineExpression {
    fn walk(&self, visitor: &mut dyn Visitor) {
        visitor.visit_inline_expression(self);
        match self {
            Self::StringLiteral(literal) => Walker::walk(literal, visitor),
            Self::NumberLiteral(literal) => Walker::walk(literal, visitor),
            Self::FunctionReference(reference) => Walker::walk(reference, visitor),
            Self::MessageReference(reference) => Walker::walk(reference, visitor),
            Self::TermReference(reference) => Walker::walk(reference, visitor),
            Self::VariableReference(reference) => Walker::walk(reference, visitor),
            Self::InlinePlaceable(inline) => Walker::walk(inline.as_ref(), visitor),
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
