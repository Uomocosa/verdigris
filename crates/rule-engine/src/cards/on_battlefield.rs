use bevy::prelude::Component;

use crate::rules;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct OnBattlefield;

impl rules::traits::keyed_component::KeyedComponent for OnBattlefield {
    const KEY: rules::ComponentKey = rules::ComponentKey("on-battlefield");
}
