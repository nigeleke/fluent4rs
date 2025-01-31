use super::prelude::{InlineExpression, VariantList};

#[cfg(feature = "walker")]
use crate::walker::{Visitor, Walkable};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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
}

#[cfg(feature = "walker")]
impl Walkable for SelectExpression {
    fn walk(&self, visitor: &mut dyn Visitor) {
        visitor.visit_select_expression(self);
        self.inline_expression.walk(visitor);
        self.variant_list.walk(visitor);
    }
}

impl std::fmt::Display for SelectExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.inline_expression, self.variant_list)
    }
}
