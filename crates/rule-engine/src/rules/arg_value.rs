use bevy::prelude::Entity;
use derive_more::From;

use crate::cards;
use crate::players;
use crate::rules;

#[derive(Debug, Clone, PartialEq, Eq, Hash, From)]
pub enum ArgValue {
    Entity(Entity),
    Entities(Vec<Entity>),
    Player(players::PlayerId),
    Amount(u32),
    Event(rules::EventId),
}

#[rustfmt::skip]
impl From<cards::CardId> for ArgValue {
    fn from(card: cards::CardId) -> Self { Self::Entity(*card) }
}
