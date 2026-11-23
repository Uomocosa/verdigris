use bevy::prelude::Component;

use crate::rules;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Defender;

impl rules::traits::keyed_component::KeyedComponent for Defender {
    const KEY: rules::ComponentKey = rules::ComponentKey("defender");
}
