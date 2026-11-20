use bevy::prelude::{Component, Entity};
use derive_more::Deref;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Deref)]
pub struct Owner(pub Entity);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Attacking;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Deref)]
pub struct Blocking(pub Entity);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Blocked;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tapped;
