use bevy::prelude::{Component, Entity};
use derive_more::Deref;

use crate::effects;

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct ContinuousEffect {
    pub affected: Entity,
    pub source: Option<Entity>,
    pub modification: effects::Modification,
    pub until: effects::Duration,
    pub timestamp: u64,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Deref)]
pub struct SourceTimestamp(pub u64);
