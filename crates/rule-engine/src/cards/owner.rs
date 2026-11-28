use bevy::prelude::{Component, Entity};
use derive_more::Deref;

use crate::players;
use crate::rules;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Deref)]
pub struct Owner(pub players::PlayerId);

#[rustfmt::skip]
impl rules::traits::entity_component::EntityComponent for Owner {
    fn targets(&self) -> Vec<Entity> { vec![***self] }
}
