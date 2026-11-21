use bevy::prelude::{Component, Entity};
use derive_more::Deref;

use crate::effects;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Deref)]
pub struct ChosenTarget(pub Entity);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CastingStage {
    AwaitingTargets,
    AwaitingPayment,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct TriggeredAbilityOnStack {
    pub source: Entity,
    pub controller: Entity,
    pub effect: effects::Effect,
}
