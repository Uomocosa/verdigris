use crate::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Term {
    Var(&'static str),
    Val(Value),
}
