use bevy::prelude::{Entity, Message};

use crate::actions;
use crate::players;
use crate::rules;

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct Ask {
    pub player: players::PlayerId,
    pub event: rules::EventId,
    pub option: Entity,
}

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct Answered {
    pub player: players::PlayerId,
    pub event: rules::EventId,
    pub option: Entity,
    pub is_accepted: bool,
}

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct PlayerAction {
    pub player: players::PlayerId,
    pub action: actions::Action,
}

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub enum Rejected {
    CannotActivateManaAbility {
        permanent: Entity,
    },
    NoLegalTargets {
        spell: Entity,
    },
    CannotCast {
        spell: Entity,
    },
    CannotPlayLand {
        land: Entity,
    },
    CannotPay {
        spell: Entity,
    },
    InvalidTargets {
        spell: Entity,
    },
    InvalidDiscard {
        player: players::PlayerId,
    },
    InvalidTriggerOrder {
        player: players::PlayerId,
    },
    InvalidMay {
        player: players::PlayerId,
    },
    InvalidCombatDamage {
        player: players::PlayerId,
    },
    InvalidGraveyardOrder {
        player: players::PlayerId,
    },
    CannotChooseDefendingPlayer {
        player: players::PlayerId,
    },
    CannotAttack {
        attackers: Vec<Entity>,
    },
    CannotBlock {
        assignments: Vec<actions::BlockAssignment>,
    },
    NoPriority {
        player: players::PlayerId,
    },
}
