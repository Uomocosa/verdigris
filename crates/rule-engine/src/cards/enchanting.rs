use bevy::prelude::{Component, Entity};
use derive_more::Deref;

use crate::rules;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Deref)]
pub struct Enchanting(pub Entity);

impl rules::traits::keyed_component::KeyedComponent for Enchanting {
    const KEY: rules::ComponentKey = rules::ComponentKey("enchanting");
}

#[rustfmt::skip]
impl rules::traits::entity_component::EntityComponent for Enchanting {
    fn target(&self) -> Entity { **self }
}
