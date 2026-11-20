use bevy::prelude::Entity;

use crate::cards;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestAbility {
    Only {
        source: Entity,
    },
    Mana {
        source: Entity,
        produces: cards::Color,
    },
}
