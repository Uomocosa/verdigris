use derive_more::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref)]
pub struct ChoiceId(pub &'static str);
