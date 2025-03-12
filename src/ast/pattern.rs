use super::PatternElement;

#[cfg(feature = "walker")]
use crate::walker::{Visitor, Walkable, Walker};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [Pattern](crate::ast::Pattern) ::= [PatternElement](crate::ast::PatternElement)+
///
/// [Pattern](crate::ast::Pattern)s are values of [Message](crate::ast::Message)s,
/// [Term](crate::ast::Term)s, [Attribute](crate::ast::Attribute)s and [Variant](crate::ast::Variant)s.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Pattern(Vec<PatternElement>);

impl Pattern {
    pub fn pattern_elements(&self) -> &[PatternElement] {
        self.0.as_slice()
    }
}

impl From<&[PatternElement]> for Pattern {
    fn from(value: &[PatternElement]) -> Self {
        Self(Vec::from(value))
    }
}

#[cfg(feature = "walker")]
impl Walkable for Pattern {
    fn walk(&self, visitor: &mut dyn Visitor) {
        visitor.visit_pattern(self);
        self.0.iter().for_each(|pe| Walker::walk(pe, visitor));
    }
}

impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self
            .0
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join("");
        write!(f, "{value}")
    }
}
