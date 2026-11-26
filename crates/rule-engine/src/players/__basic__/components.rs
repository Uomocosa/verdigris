use bevy::prelude::Component;
use derive_more::Deref;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct DefendingPlayer;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deref)]
pub struct Life(pub i32);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Deref)]
pub struct PoisonCounters(pub u32);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Lost;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct LeftTheGame;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Won;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameIsADraw;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct AttemptedDrawFromEmptyLibrary;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Player;
