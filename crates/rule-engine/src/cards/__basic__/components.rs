use bevy::prelude::{Component, Entity};
use derive_more::Deref;

use crate::abilities;
use crate::cards;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Deref)]
pub struct Owner(pub Entity);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Deref)]
pub struct Blocking(pub Entity);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Deref)]
pub struct Enchanting(pub Entity);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Blocked;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct OnBattlefield;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Deref)]
pub struct EffectivePT(pub cards::PowerToughness);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Indestructible;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Deathtouch;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Infect;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Lifelink;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Trample;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Deref)]
pub struct TriggeredEnters(pub abilities::TriggeredAbility);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Deref)]
pub struct TriggeredAtStep(pub abilities::TriggeredAbility);
