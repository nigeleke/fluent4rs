#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
pub struct Identifier(String);

impl From<&str> for Identifier {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
