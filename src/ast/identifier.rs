#[derive(Clone, Debug)]
pub struct Identifier(String);

impl From<String> for Identifier {
    fn from(value: String) -> Self {
        Self(value)
    }
}
