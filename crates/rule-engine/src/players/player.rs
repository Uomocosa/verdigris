use bevy::prelude::Component;

use crate::rules;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Player;

impl rules::traits::keyed_component::KeyedComponent for Player {
    const KEY: rules::ComponentKey = rules::ComponentKey("player");
}
