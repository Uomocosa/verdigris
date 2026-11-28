use bevy::prelude::{Component, Entity};
use derive_more::Deref;

use crate::players;
use crate::rules;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Deref)]
pub struct Controller(pub players::PlayerId);

#[rustfmt::skip]
impl rules::traits::entity_component::EntityComponent for Controller {
    fn targets(&self) -> Vec<Entity> { vec![***self] }
}
