use bevy::prelude::{Entity, Message};

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct PowerToughnessModified {
    pub target: Entity,
    pub power: i32,
    pub toughness: i32,
}
