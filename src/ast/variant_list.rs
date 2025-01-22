use super::{prelude::DefaultVariant, variant::Variant};

#[derive(Clone, Debug, PartialEq)]
pub struct VariantList {
    pre_default: Vec<Variant>,
    default: DefaultVariant,
    post_default: Vec<Variant>,
}

impl VariantList {
    pub fn new(
        pre_default: Vec<Variant>,
        default: DefaultVariant,
        post_default: Vec<Variant>,
    ) -> Self {
        Self {
            pre_default,
            default,
            post_default,
        }
    }
}
