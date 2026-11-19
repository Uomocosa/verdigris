use bevy::prelude::{Entity, Resource};
use derive_more::Deref;

use crate::turns;

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Turn {
    pub number: u32,
    pub active: Entity,
    pub phase: turns::Phase,
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Deref)]
pub struct Priority(pub Entity);
