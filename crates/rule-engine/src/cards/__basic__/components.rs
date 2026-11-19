use bevy::prelude::{Component, Entity};
use derive_more::Deref;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Deref)]
pub struct Owner(pub Entity);
