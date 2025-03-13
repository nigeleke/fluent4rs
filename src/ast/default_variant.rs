use super::{Pattern, VariantKey};

#[cfg(feature = "walker")]
use crate::walker::{Visitor, Walkable, Walker};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [DefaultVariant](crate::ast::DefaultVariant) ::= line_end blank? "*" [VariantKey](crate::ast::VariantKey) blank_inline? [Pattern](crate::ast::Pattern)
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

    pub fn variant_key(&self) -> &VariantKey {
        &self.variant_key
    }

    pub fn variant_key_name(&self) -> String {
        format!("*[{}]", self.variant_key)
    }

    pub fn pattern(&self) -> &Pattern {
        &self.pattern
    }
}

#[cfg(feature = "walker")]
impl Walkable for DefaultVariant {
    fn walk(&self, visitor: &mut dyn Visitor) {
        visitor.visit_default_variant(self);
        Walker::walk(&self.variant_key, visitor);
        Walker::walk(&self.pattern, visitor);
    }
}

impl std::fmt::Display for DefaultVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n   {} {}", self.variant_key_name(), self.pattern)
    }
}
