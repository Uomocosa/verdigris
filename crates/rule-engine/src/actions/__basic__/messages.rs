use bevy::prelude::{Entity, Message};
use derive_more::Deref;

use crate::actions;
use crate::players;
use crate::rules;

#[derive(Message, Debug, Clone, PartialEq, Eq, Deref)]
pub struct Ask(pub actions::Choice);

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct Answered {
    pub choice: actions::Choice,
    pub answer: actions::Action,
}

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct DiscardRequest {
    pub player: players::PlayerId,
    pub count: u32,
    pub cause: rules::EventId,
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
    InvalidChoice {
        player: players::PlayerId,
        why: actions::ChoiceWhy,
    },
    InvalidCombatDamage {
        player: players::PlayerId,
    },
    CannotAttack {
        attackers: Vec<Entity>,
        why: Vec<actions::DeclarationWhy>,
    },
    CannotBlock {
        assignments: Vec<actions::BlockAssignment>,
    },
    NoPriority {
        player: players::PlayerId,
    },
    NothingToChoose {
        player: players::PlayerId,
    },
}
