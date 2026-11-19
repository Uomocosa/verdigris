use bevy::prelude::Component;

#[derive(Component, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PowerToughnessModifier {
    pub power: i32,
    pub toughness: i32,
}
