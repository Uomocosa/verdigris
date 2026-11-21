use bevy::prelude::Entity;

use crate::actions;
use crate::cards;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    ActivateManaAbility {
        source: Entity,
        produces: cards::Color,
    },
    Cast(Entity),
    PlayLand(Entity),
    ChooseTargets(Vec<Entity>),
    ChooseDiscard(Vec<Entity>),
    ChooseLegendary(Entity),
    ChooseDefendingPlayer(Entity),
    OrderTriggers(Vec<Entity>),
    AssignCombatDamage(Vec<actions::DamageAssignment>),
    DeclareAttackers(Vec<Entity>),
    DeclareBlockers(Vec<actions::BlockAssignment>),
    PassPriority,
    Concede,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Choice {
    Target(actions::TargetChoice),
    Discard(actions::DiscardChoice),
    CombatDamage(actions::CombatDamageChoice),
    OrderTriggers(actions::OrderTriggersChoice),
    Legendary(actions::LegendaryChoice),
}
