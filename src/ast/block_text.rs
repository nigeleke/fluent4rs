#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Hash, Eq))]
pub struct BlockText(String);

impl From<&str> for BlockText {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl std::fmt::Display for BlockText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
