use bevy::prelude::{Component, Entity};
use derive_more::Deref;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Deref)]
#[relationship(relationship_target = ZoneContents)]
pub struct InZone(pub Entity);

#[derive(Component, Debug, Clone, PartialEq, Eq, Deref)]
#[relationship_target(relationship = InZone)]
pub struct ZoneContents(Vec<Entity>);
