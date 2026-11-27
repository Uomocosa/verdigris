use bevy::prelude::Entity;

use std::collections::{BTreeMap, BTreeSet};

use crate::players;
use crate::rules;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockAssignment {
    pub blocker: Entity,
    pub attacker: Entity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DamageAssignment {
    pub target: Entity,
    pub amount: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequiredCombatDamage {
    pub source: Entity,
    pub amount: u32,
    pub legal: BTreeSet<Entity>,
    pub trample_defender: Option<Entity>,
    pub lethal_by_target: BTreeMap<Entity, u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderTriggersChoice {
    pub player: players::PlayerId,
    pub triggers: Vec<Entity>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MayChoice {
    pub player: players::PlayerId,
    pub event: rules::EventId,
    pub options: Vec<Entity>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrangeGraveyardChoice {
    pub player: players::PlayerId,
    pub graveyard: Entity,
    pub cards: Vec<Entity>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefendingPlayerChoice {
    pub player: players::PlayerId,
    pub legal: Vec<players::PlayerId>,
}
