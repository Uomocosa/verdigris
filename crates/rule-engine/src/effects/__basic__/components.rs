use bevy::prelude::{Component, Entity};

use crate::effects;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContinuousEffect {
    pub affected: Entity,
    pub change: effects::Layered,
    pub until: effects::Duration,
    pub timestamp: u64,
}
