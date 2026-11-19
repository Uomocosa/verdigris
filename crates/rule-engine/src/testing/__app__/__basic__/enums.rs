use bevy::prelude::Entity;

use crate::actions;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayStep {
    Act(Entity, actions::Action),
    PassPriorityAll,
}
