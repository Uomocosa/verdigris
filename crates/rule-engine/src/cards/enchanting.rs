use bevy::prelude::{Component, Entity};
use derive_more::Deref;

use crate::rules;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Deref)]
pub struct Enchanting(pub Entity);

#[rustfmt::skip]
impl rules::traits::entity_component::EntityComponent for Enchanting {
    fn targets(&self) -> Vec<Entity> { vec![**self] }
}
