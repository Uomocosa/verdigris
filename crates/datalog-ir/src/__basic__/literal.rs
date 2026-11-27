use crate::Atom;
use crate::CmpOp;
use crate::Term;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Literal {
    Pos(Atom),
    Neg(Atom),
    Cmp(Term, CmpOp, Term),
}
