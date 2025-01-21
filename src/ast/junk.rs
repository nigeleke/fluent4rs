#[derive(Debug)]
pub struct Junk(Vec<String>);

impl From<&[String]> for Junk {
    fn from(value: &[String]) -> Self {
        Self(Vec::from(value))
    }
}
