#[derive(Clone, Debug)]
pub struct StringLiteral(String);

impl From<&str> for StringLiteral {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}
