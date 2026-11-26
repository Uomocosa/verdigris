use bevy::prelude::{Component, Entity};
use derive_more::Deref;

use crate::players;
use crate::rules;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Deref)]
pub struct Controller(pub players::PlayerId);

impl rules::traits::keyed_component::KeyedComponent for Controller {
    const KEY: rules::ComponentKey = rules::ComponentKey("controller");
}

#[rustfmt::skip]
impl rules::traits::entity_component::EntityComponent for Controller {
    fn target(&self) -> Entity { ***self }
}
