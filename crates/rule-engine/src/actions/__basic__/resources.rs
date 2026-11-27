use bevy::prelude::Resource;
use derive_more::{Deref, DerefMut};

use crate::May;
use crate::actions;
use crate::players;
use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Resource, Debug, Deref, DerefMut)]
pub struct PendingMayAsks<E: Event>(pub Vec<May<E>>);

#[derive(Resource, Debug, Deref, DerefMut)]
pub struct AcceptedMays<E: Event>(pub Vec<May<E>>);

#[derive(Resource, Debug, Clone, Default, PartialEq, Eq)]
pub struct LegalActions {
    pub player: Option<players::PlayerId>,
    pub actions: Vec<(actions::Action, rules::Legality)>,
}
