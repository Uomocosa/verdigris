use bevy::prelude::{Entity, Message};

use crate::actions;

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct PlayerAction {
    pub player: Entity,
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
        player: Entity,
    },
    InvalidCombatDamage {
        player: Entity,
    },
    CannotAttack {
        attackers: Vec<Entity>,
    },
    CannotBlock {
        assignments: Vec<actions::BlockAssignment>,
    },
    NoPriority {
        player: Entity,
    },
}
