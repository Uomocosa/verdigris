use crate::Atom;
use crate::Literal;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rule {
    pub head: Atom,
    pub body: Vec<Literal>,
}
