use bevy::prelude::{Entity, Message};
use derive_more::Deref;

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq, Deref)]
pub struct Resolution(pub Entity);
