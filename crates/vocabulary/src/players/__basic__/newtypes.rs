use bevy::prelude::Entity;
use derive_more::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deref)]
pub struct PlayerId(pub Entity);
