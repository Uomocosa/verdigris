use bevy::prelude::Entity;

use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetChoice {
    pub spell: Entity,
    pub count: u32,
    pub legal: BTreeSet<Entity>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscardChoice {
    pub player: Entity,
    pub count: u32,
    pub legal: BTreeSet<Entity>,
}

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
    pub player: Entity,
    pub triggers: Vec<Entity>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegendaryChoice {
    pub player: Entity,
    pub legal: BTreeSet<Entity>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrangeGraveyardChoice {
    pub player: Entity,
    pub graveyard: Entity,
    pub cards: Vec<Entity>,
}
