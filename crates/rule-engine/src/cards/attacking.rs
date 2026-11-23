use bevy::prelude::Component;

use crate::rules;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Attacking;

impl rules::traits::keyed_component::KeyedComponent for Attacking {
    const KEY: rules::ComponentKey = rules::ComponentKey("attacking");
}
