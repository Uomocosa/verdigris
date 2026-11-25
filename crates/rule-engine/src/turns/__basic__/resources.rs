use std::collections::BTreeSet;

use bevy::prelude::{Entity, Resource};
use derive_more::{Deref, DerefMut};

#[derive(Resource, Debug, Default, Deref, DerefMut)]
pub struct CombatDamageParticipants(pub BTreeSet<Entity>);

#[derive(Resource, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CleanupReentry {
    pub is_priority_window: bool,
    pub should_repeat: bool,
    pub is_extra_cleanup: bool,
}
