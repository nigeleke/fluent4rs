use super::{Pattern, VariantKey};

#[cfg(feature = "walker")]
use crate::walker::{Visitor, Walkable, Walker};

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
    /// Constructs a new non-default `Variant` for use in a Fluent select expression.
    ///
    /// # Arguments
    /// * `variant_key` - The key (selector value) for this variant. Typically a literal string,
    ///                   number, or plural category (e.g., `"one"`, `"few"`, `"many"`).
    /// * `pattern` - The message pattern displayed when this variant is selected.
    pub fn new(variant_key: VariantKey, pattern: Pattern) -> Self {
        Self {
            variant_key,
            pattern,
        }
    }

    /// Returns a reference to the variant key of this variant.
    pub fn variant_key(&self) -> &VariantKey {
        &self.variant_key
    }

    /// Returns a string representation of the variant key as it appears in Fluent source
    /// (without the asterisk), e.g., `[one]`, `[few]`.
    ///
    /// This is useful when serializing or pretty-printing the variant.
    /// Note that default variants include a leading `*` — use `DefaultVariant::variant_key_name`
    /// for those.
    pub fn variant_key_name(&self) -> String {
        format!("[{}]", self.variant_key)
    }

    /// Returns a reference to the pattern of this variant.
    pub fn pattern(&self) -> &Pattern {
        &self.pattern
    }
}

#[cfg(feature = "walker")]
impl Walkable for Variant {
    fn walk(&self, visitor: &mut dyn Visitor) {
        visitor.visit_variant(self);
        Walker::walk(&self.variant_key, visitor);
        Walker::walk(&self.pattern, visitor);
    }
}

impl std::fmt::Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n    {} {}", self.variant_key_name(), self.pattern)
    }
}
