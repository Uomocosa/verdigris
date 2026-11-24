use bevy::prelude::Message;
use derive_more::Deref;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Message, Clone, Debug, Deref)]
pub struct Would<E>(pub E);

#[derive(Message, Clone, Debug, Deref)]
pub struct Proceeds<E>(pub E);

#[derive(Message, Clone, Debug)]
pub struct Ask<E: Event> {
    pub event: E,
    pub then: rules::EventTemplate,
}
