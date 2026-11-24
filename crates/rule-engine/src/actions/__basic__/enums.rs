use bevy::prelude::Entity;

use crate::actions;
use crate::cards;
use crate::players;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    ActivateManaAbility {
        source: Entity,
        produces: cards::ManaType,
    },
    Cast(Entity),
    PlayLand(Entity),
    ChooseTargets(Vec<Entity>),
    ChooseMay(Vec<Entity>),
    ChooseDiscard(Vec<Entity>),
    ChooseLegendary(Entity),
    ChooseDefendingPlayer(players::PlayerId),
    OrderTriggers(Vec<Entity>),
    ArrangeGraveyard(Vec<Entity>),
    AssignCombatDamage(Vec<actions::DamageAssignment>),
    DeclareAttackers(Vec<Entity>),
    DeclareBlockers(Vec<actions::BlockAssignment>),
    PassPriority,
    Concede,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Choice {
    Target(actions::TargetChoice),
    May(actions::MayChoice),
    Discard(actions::DiscardChoice),
    CombatDamage(actions::CombatDamageChoice),
    OrderTriggers(actions::OrderTriggersChoice),
    ArrangeGraveyard(actions::ArrangeGraveyardChoice),
    Legendary(actions::LegendaryChoice),
}
