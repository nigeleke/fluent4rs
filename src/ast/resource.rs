use super::prelude::{Entry, Junk};

#[derive(Debug)]
pub enum Resource {
    Entry(Entry),
    BlankBlock(String),
    Junk(Junk),
}
