use super::prelude::PatternElement;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Hash))]
pub struct Pattern(Vec<PatternElement>);

impl From<&[PatternElement]> for Pattern {
    fn from(value: &[PatternElement]) -> Self {
        Self(Vec::from(value))
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
