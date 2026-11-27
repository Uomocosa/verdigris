use bevy::prelude::Entity;

use std::collections::{BTreeMap, BTreeSet};

use crate::rules;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeclarationWhy {
    pub creature: Option<Entity>,
    pub why: rules::Why,
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
