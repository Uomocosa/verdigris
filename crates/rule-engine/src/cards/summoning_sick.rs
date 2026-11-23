use bevy::prelude::Component;

use crate::rules;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct SummoningSick;

impl rules::traits::keyed_component::KeyedComponent for SummoningSick {
    const KEY: rules::ComponentKey = rules::ComponentKey("summoning-sick");
}
