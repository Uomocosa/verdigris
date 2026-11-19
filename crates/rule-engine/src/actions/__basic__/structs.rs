use bevy::prelude::Entity;

use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetChoice {
    pub spell: Entity,
    pub count: u32,
    pub legal: BTreeSet<Entity>,
}
