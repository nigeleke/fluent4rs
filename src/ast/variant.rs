use super::{pattern::Pattern, prelude::VariantKey};

#[derive(Clone, Debug)]
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
