use bevy::prelude::{Entity, Resource};
use derive_more::{Deref, DerefMut};

use crate::actions;

#[derive(Resource, Debug, Default, Deref)]
pub struct PendingChoice(pub Option<actions::Choice>);

#[derive(Resource, Debug, Default, Deref, DerefMut)]
pub struct OpeningHandWindow(pub Vec<Entity>);
