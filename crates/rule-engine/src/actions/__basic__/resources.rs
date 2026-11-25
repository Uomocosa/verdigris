use bevy::prelude::Resource;
use derive_more::{Deref, DerefMut};

use crate::May;
use crate::actions;
use crate::rules::traits::event::Event;

#[derive(Resource, Debug, Default, Deref)]
pub struct PendingChoice(pub Option<actions::Choice>);

#[derive(Resource, Debug, Default, Deref, DerefMut)]
pub struct AskQueue(pub Vec<actions::Ask>);

#[derive(Resource, Debug, Deref, DerefMut)]
pub struct PendingMayAsks<E: Event>(pub Vec<May<E>>);

#[derive(Resource, Debug, Deref, DerefMut)]
pub struct AcceptedMays<E: Event>(pub Vec<May<E>>);
