use bevy::prelude::Message;
use derive_more::Deref;

#[derive(Message, Clone, Debug, Deref)]
pub struct Would<E>(pub E);

#[derive(Message, Clone, Debug, Deref)]
pub struct Proceeds<E>(pub E);
