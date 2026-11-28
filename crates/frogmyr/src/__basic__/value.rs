#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::From)]
pub enum Value {
    Entity(u64),
    Sym(&'static str),
    Int(i64),
}
