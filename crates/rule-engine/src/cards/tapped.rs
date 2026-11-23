use bevy::prelude::Component;

use crate::rules;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tapped;

impl rules::traits::keyed_component::KeyedComponent for Tapped {
    const KEY: rules::ComponentKey = rules::ComponentKey("tapped");
}
