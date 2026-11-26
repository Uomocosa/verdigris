use derive_more::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref)]
pub struct EventId(pub &'static str);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref)]
pub struct RuleId(pub &'static str);
