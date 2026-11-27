#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Value {
    Entity(u64),
    Sym(&'static str),
    Int(i64),
}
