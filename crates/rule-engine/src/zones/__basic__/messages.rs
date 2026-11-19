use bevy::prelude::{Entity, Message};
use derive_more::Deref;

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq, Deref)]
pub struct Destruction(pub Entity);

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct DrawRequested {
    pub player: Entity,
    pub count: u32,
}

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct DamageDealt {
    pub source: Entity,
    pub target: Entity,
    pub amount: u32,
}
