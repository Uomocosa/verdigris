use bevy::prelude::{Entity, Resource};

use crate::turns;

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Turn {
    pub number: u32,
    pub active: Entity,
    pub phase: turns::Phase,
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Priority {
    pub player: Entity,
    pub consecutive_passes: u32,
}
