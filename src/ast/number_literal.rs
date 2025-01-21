#[derive(Clone, Debug)]
pub struct NumberLiteral(String);

impl From<&str> for NumberLiteral {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}
