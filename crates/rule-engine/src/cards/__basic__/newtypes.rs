use derive_more::{Deref, From};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, From)]
#[from(forward)]
pub struct CardName(pub String);
