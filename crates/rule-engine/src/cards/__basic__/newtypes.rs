use bevy::prelude::Entity;
use derive_more::{Deref, From};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deref)]
pub struct CardId(pub Entity);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, From)]
#[from(forward)]
pub struct CardName(pub String);
