use bevy::prelude::Component;

use crate::rules;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Reach;

impl rules::traits::keyed_component::KeyedComponent for Reach {
    const KEY: rules::ComponentKey = rules::ComponentKey("reach");
}
