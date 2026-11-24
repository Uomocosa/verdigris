use bevy::prelude::Resource;
use derive_more::{Deref, DerefMut};

#[derive(Resource, Debug, Default, Deref, DerefMut)]
pub struct StateBasedActionsPerformed(pub bool);

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameEnded;
