use bevy::prelude::{Component, Entity};
use derive_more::Deref;

use crate::rules;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Deref)]
pub struct Blocking(pub Entity);

#[rustfmt::skip]
impl rules::traits::entity_component::EntityComponent for Blocking {
    fn target(&self) -> Entity { **self }
}
