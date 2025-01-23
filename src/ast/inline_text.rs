#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Hash, Eq))]
pub struct InlineText(String);

impl From<&str> for InlineText {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl std::fmt::Display for InlineText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
