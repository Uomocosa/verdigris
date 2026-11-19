use bevy::prelude::Entity;

use crate::actions;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Step {
    Act(Entity, actions::Action),
    PassPriorityAll,
}
