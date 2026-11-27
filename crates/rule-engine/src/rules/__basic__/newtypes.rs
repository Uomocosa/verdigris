use derive_more::Deref;

use crate::rules;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref)]
pub struct EventId(pub &'static str);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref)]
pub struct ResultId(pub &'static str);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref)]
pub struct RuleId(pub &'static str);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref)]
pub struct An(pub rules::Players);
