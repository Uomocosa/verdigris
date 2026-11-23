use bevy::prelude::Component;

use crate::rules;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Creature;

impl rules::traits::keyed_component::KeyedComponent for Creature {
    const KEY: rules::ComponentKey = rules::ComponentKey("creature");
}
