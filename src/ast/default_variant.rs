use super::{pattern::Pattern, variant_key::VariantKey};

#[derive(Clone, Debug, PartialEq)]
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

impl std::fmt::Display for DefaultVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n *[{}] {}", self.variant_key, self.pattern)
    }
}
