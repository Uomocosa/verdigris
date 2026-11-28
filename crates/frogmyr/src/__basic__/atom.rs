use crate::Rel;
use crate::Term;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Atom {
    pub rel: Rel,
    pub terms: Vec<Term>,
}
