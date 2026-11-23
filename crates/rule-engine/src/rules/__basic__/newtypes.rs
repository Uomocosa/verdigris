use derive_more::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Deref)]
pub struct ComponentKey(pub &'static str);
