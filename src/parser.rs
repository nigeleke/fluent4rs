use super::ast::prelude::Resource;
use super::error::Error;

pub struct Parser;

impl Parser {
    pub fn parse(text: &str) -> Result<Resource, Error> {
        let resource_parser = super::grammar::resource();

        // .parse(text.as_bytes())
        // .map_err(|e| Error::FailedToParse(e.to_string()));
        Err(Error::FailedToParse("asd".into()))
    }
}
