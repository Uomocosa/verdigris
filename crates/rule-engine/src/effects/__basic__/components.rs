use bevy::prelude::{Component, Entity};
use derive_more::Deref;

use crate::effects;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContinuousEffect {
    pub affected: Entity,
    pub source: Option<Entity>,
    pub change: effects::Change,
    pub until: effects::Duration,
    pub timestamp: u64,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Deref)]
pub struct SourceTimestamp(pub u64);
