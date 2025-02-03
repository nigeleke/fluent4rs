use super::{DefaultVariant, Variant};

#[cfg(feature = "walker")]
use crate::walker::{Visitor, Walkable};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// variant_list ::= [Variant](crate::ast::Variant)* [DefaultVariant](crate::ast::DefaultVariant) [Variant](crate::ast::Variant)* line_end
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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

#[cfg(feature = "walker")]
impl Walkable for VariantList {
    fn walk(&self, visitor: &mut dyn Visitor) {
        self.pre_default.iter().for_each(|v| v.walk(visitor));
        self.default.walk(visitor);
        self.post_default.iter().for_each(|v| v.walk(visitor));
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
