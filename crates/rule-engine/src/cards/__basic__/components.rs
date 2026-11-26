use bevy::prelude::Component;
use derive_more::Deref;

use crate::abilities;
use crate::cards;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Blocked;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Deref)]
pub struct EffectivePT(pub cards::PowerToughness);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct HadFirstStrikeAsDamageBegan;

#[derive(Component, Debug, Clone, PartialEq, Eq, Deref)]
pub struct TriggeredEnters(pub abilities::TriggeredAbility);

#[derive(Component, Debug, Clone, PartialEq, Eq, Deref)]
pub struct TriggeredAtStep(pub abilities::TriggeredAbility);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Attacking;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Creature;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct OnBattlefield;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct SummoningSick;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tapped;
