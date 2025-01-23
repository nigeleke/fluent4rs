use super::prelude::{InlineExpression, VariantList};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Hash))]
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

impl std::fmt::Display for SelectExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.inline_expression, self.variant_list)
    }
}
