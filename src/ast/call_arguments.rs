use super::prelude::Argument;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
pub struct CallArguments(Vec<Argument>);

impl From<&[Argument]> for CallArguments {
    fn from(value: &[Argument]) -> Self {
        Self(Vec::from(value))
    }
}

impl std::fmt::Display for CallArguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // argument_list       ::= (Argument blank? "," blank?)* Argument?
        let arguments = self
            .0
            .iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "({arguments})")
    }
}
