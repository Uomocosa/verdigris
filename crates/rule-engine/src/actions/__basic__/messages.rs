use bevy::prelude::{Entity, Message};
use derive_more::Deref;

use crate::actions;
use crate::players;

#[derive(Message, Debug, Clone, PartialEq, Eq, Deref)]
pub struct Ask(pub actions::Choice);

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct Answered {
    pub choice: actions::Choice,
    pub answer: actions::Action,
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
    CannotPlay {
        card: Entity,
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
