use bevy::prelude::Entity;

use crate::actions;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Cast(Entity),
    ChooseTargets(Vec<Entity>),
    ChooseDiscard(Vec<Entity>),
    DeclareAttackers(Vec<Entity>),
    PassPriority,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Choice {
    Target(actions::TargetChoice),
    Discard(actions::DiscardChoice),
}
