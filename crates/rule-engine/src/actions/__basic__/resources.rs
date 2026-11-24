use bevy::prelude::Resource;
use derive_more::{Deref, DerefMut};

use crate::actions;
use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Resource, Debug, Default, Deref)]
pub struct PendingChoice(pub Option<actions::Choice>);

#[derive(Resource, Debug, Default, Deref, DerefMut)]
pub struct AskQueue(pub Vec<actions::Ask>);

#[derive(Resource, Debug, Deref, DerefMut)]
pub struct PendingMayAsks<E: Event>(pub Vec<rules::Ask<E>>);
