use bevy::prelude::{Entity, Message};

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct LifeGained {
    pub player: Entity,
    pub amount: u32,
}

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiscardRequested {
    pub player: Entity,
    pub count: u32,
}
