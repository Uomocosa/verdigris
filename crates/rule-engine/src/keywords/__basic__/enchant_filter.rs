use derive_more::Deref;

use crate::effects;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref)]
pub struct EnchantFilter(pub effects::Filter);
