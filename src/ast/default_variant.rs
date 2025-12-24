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
    /// Constructs a new `DefaultVariant`.
    /// The default variant is the fallback used when no other variant matches the selector value.
    ///
    /// # Arguments
    /// * `variant_key` - The key (selector value) for this variant. Typically a literal number or string.
    /// * `pattern` - The message pattern displayed when this variant is selected as the fallback.
    pub fn new(variant_key: VariantKey, pattern: Pattern) -> Self {
        Self {
            variant_key,
            pattern,
        }
    }

    /// Returns a reference to the variant key of this default variant.
    pub fn variant_key(&self) -> &VariantKey {
        &self.variant_key
    }

    /// Returns a string representation of the variant key as it appears in Fluent source,
    /// including the leading asterisk (e.g., `*[zero]`, `*[other]`).
    ///
    /// This is useful when serializing or pretty-printing the variant.
    pub fn variant_key_name(&self) -> String {
        format!("*[{}]", self.variant_key)
    }

    /// Returns a reference to the pattern of this default variant.
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
