use std::marker::PhantomData;

use bevy::prelude::Message;
use derive_more::Deref;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Message, Clone, Debug, Deref)]
pub struct Resolve<E>(pub E);

#[derive(Message, Clone, Debug, Deref)]
pub struct Did<E>(pub E);

#[derive(Message, Clone, Debug)]
pub struct May<E: Event> {
    pub event: E,
    pub then: rules::EventTemplate,
}

#[derive(Message, Clone, Copy, Debug, Deref, PartialEq, Eq)]
pub struct HooksDone<E>(pub PhantomData<E>);
