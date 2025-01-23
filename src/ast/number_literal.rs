#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Hash))]
pub struct NumberLiteral(String);

impl From<&str> for NumberLiteral {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl std::fmt::Display for NumberLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
