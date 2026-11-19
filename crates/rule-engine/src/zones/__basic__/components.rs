use bevy::prelude::{Component, Entity};
use derive_more::Deref;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Zone {
    Library,
    Hand,
    Battlefield,
    Graveyard,
    Exile,
    Stack,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Deref)]
#[relationship(relationship_target = ZoneContents)]
pub struct InZone(pub Entity);

#[derive(Component, Debug, Clone, PartialEq, Eq, Deref)]
#[relationship_target(relationship = InZone)]
pub struct ZoneContents(Vec<Entity>);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Deref)]
pub struct Owner(pub Entity);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Player;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deref)]
pub struct Life(pub i32);
