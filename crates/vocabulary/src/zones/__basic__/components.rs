use bevy::prelude::{Component, Entity};
use derive_more::{Deref, DerefMut};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Deref)]
#[relationship(relationship_target = ZoneContents)]
pub struct InZone(pub Entity);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Deref)]
pub struct LeftZone(pub Entity);

#[derive(Component, Debug, Clone, PartialEq, Eq, Deref, DerefMut)]
#[relationship_target(relationship = InZone)]
pub struct ZoneContents(Vec<Entity>);
