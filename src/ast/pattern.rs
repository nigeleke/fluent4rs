use super::prelude::PatternElement;

#[derive(Clone, Debug)]
pub struct Pattern(Vec<PatternElement>);

impl From<&[PatternElement]> for Pattern {
    fn from(value: &[PatternElement]) -> Self {
        Self(Vec::from(value))
    }
}
