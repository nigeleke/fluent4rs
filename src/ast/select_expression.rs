use super::{InlineExpression, VariantList};

#[cfg(feature = "walker")]
use crate::walker::{Visitor, Walkable, Walker};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [SelectExpression](crate::ast::SelectExpression) ::= [InlineExpression](crate::ast::InlineExpression) blank? "->" blank_inline? variant_list
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct SelectExpression {
    inline_expression: InlineExpression,
    variant_list: VariantList,
}

impl SelectExpression {
    pub fn new(inline_expression: InlineExpression, variant_list: VariantList) -> Self {
        Self {
            inline_expression,
            variant_list,
        }
    }

    pub fn inline_expression(&self) -> &InlineExpression {
        &self.inline_expression
    }
}

#[cfg(feature = "walker")]
impl Walkable for SelectExpression {
    fn walk(&self, visitor: &mut dyn Visitor) {
        visitor.visit_select_expression(self);
        Walker::walk(&self.inline_expression, visitor);
        Walker::walk(&self.variant_list, visitor);
    }
}

impl std::fmt::Display for SelectExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.inline_expression, self.variant_list)
    }
}
