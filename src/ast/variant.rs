use super::{pattern::Pattern, prelude::VariantKey};

#[derive(Clone, Debug, PartialEq)]
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

impl std::fmt::Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n  [{}] {}", self.variant_key, self.pattern)
    }
}
