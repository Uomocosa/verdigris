use bevy::prelude::Entity;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AssignedDamage {
    pub source: Entity,
    pub target: Entity,
    pub amount: u32,
}
