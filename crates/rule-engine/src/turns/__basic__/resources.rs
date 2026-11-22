use std::collections::BTreeSet;

use bevy::prelude::{Entity, Resource};
use derive_more::{Deref, DerefMut};

use crate::turns;

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Turn {
    pub number: u32,
    pub active: Entity,
    pub step: turns::Step,
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Priority {
    pub player: Entity,
    pub consecutive_passes: u32,
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CombatDamageSteps {
    #[default]
    Single,
    FirstOfTwo,
    SecondOfTwo,
}

#[derive(Resource, Debug, Default, Deref, DerefMut)]
pub struct CombatDamageParticipants(pub BTreeSet<Entity>);
