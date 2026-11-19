use bevy::prelude::Entity;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Cast(Entity),
    ChooseTargets(Vec<Entity>),
    PassPriority,
}
