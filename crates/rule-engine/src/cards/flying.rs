use bevy::prelude::Component;

use crate::rules;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Flying;

impl rules::traits::keyed_component::KeyedComponent for Flying {
    const KEY: rules::ComponentKey = rules::ComponentKey("flying");
}
