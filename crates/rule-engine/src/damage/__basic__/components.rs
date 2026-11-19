use bevy::prelude::Component;
use derive_more::Deref;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deref)]
pub struct Damage(pub u32);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct DealtByDeathtouch;
