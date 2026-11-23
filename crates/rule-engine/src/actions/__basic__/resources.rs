use bevy::prelude::Resource;
use derive_more::Deref;

use crate::actions;

#[derive(Resource, Debug, Default, Deref)]
pub struct PendingChoice(pub Option<actions::Choice>);
