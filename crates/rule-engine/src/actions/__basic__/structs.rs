use bevy::prelude::Entity;

use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetChoice {
    pub spell: Entity,
    pub count: u32,
    pub legal: BTreeSet<Entity>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscardChoice {
    pub player: Entity,
    pub count: u32,
    pub legal: BTreeSet<Entity>,
}
