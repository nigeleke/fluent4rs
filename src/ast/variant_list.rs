use super::{prelude::DefaultVariant, variant::Variant};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Hash, Eq))]
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

impl std::fmt::Display for VariantList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pre_default = self
            .pre_default
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join("");
        let the_default = self.default.to_string();
        let post_default = self
            .post_default
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join("");
        writeln!(f, "{}{}{}", pre_default, the_default, post_default)
    }
}
