use super::{pattern::Pattern, variant_key::VariantKey};

#[cfg(feature = "walker")]
use crate::walker::{Visitor, Walkable};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct DefaultVariant {
    variant_key: VariantKey,
    pattern: Pattern,
}

impl DefaultVariant {
    pub fn new(variant_key: VariantKey, pattern: Pattern) -> Self {
        Self {
            variant_key,
            pattern,
        }
    }
}

#[cfg(feature = "walker")]
impl Walkable for DefaultVariant {
    fn walk(&self, visitor: &mut dyn Visitor) {
        visitor.visit_default_variant(self);
        self.pattern.walk(visitor);
    }
}

impl std::fmt::Display for DefaultVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n *[{}] {}", self.variant_key, self.pattern)
    }
}
