use super::{Pattern, VariantKey};

#[cfg(feature = "walker")]
use crate::walker::{Visitor, Walkable};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [Variant](crate::ast::Variant) ::= line_end blank? [VariantKey](crate::ast::VariantKey) blank_inline? [Pattern](crate::ast::Pattern)
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Variant {
    variant_key: VariantKey,
    pattern: Pattern,
}

impl Variant {
    pub fn new(variant_key: VariantKey, pattern: Pattern) -> Self {
        Self {
            variant_key,
            pattern,
        }
    }
}

#[cfg(feature = "walker")]
impl Walkable for Variant {
    fn walk(&self, visitor: &mut dyn Visitor) {
        visitor.visit_variant(self);
        self.pattern.walk(visitor);
    }
}

impl std::fmt::Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n  [{}] {}", self.variant_key, self.pattern)
    }
}
