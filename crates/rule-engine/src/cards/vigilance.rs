use bevy::prelude::Component;

use crate::rules;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vigilance;

impl rules::traits::keyed_component::KeyedComponent for Vigilance {
    const KEY: rules::ComponentKey = rules::ComponentKey("vigilance");
}
