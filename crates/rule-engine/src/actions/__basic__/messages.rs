use bevy::prelude::{Entity, Message};

use crate::actions;

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct PlayerAction {
    pub player: Entity,
    pub action: actions::Action,
}

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub enum Rejected {
    NoLegalTargets { spell: Entity },
    CannotCast { spell: Entity },
    CannotPay { spell: Entity },
    InvalidTargets { spell: Entity },
    NoPriority { player: Entity },
}
