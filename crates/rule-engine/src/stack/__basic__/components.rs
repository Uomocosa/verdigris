use bevy::prelude::{Component, Entity};
use derive_more::Deref;

use crate::effects;
use crate::rules;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Deref)]
pub struct ChosenTarget(pub Entity);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CastingStage {
    AwaitingTargets,
    AwaitingPayment,
}

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct TriggeredAbilityOnStack {
    pub source: Entity,
    pub intervening_if: Option<rules::Condition>,
    pub effect: effects::Effect,
}
