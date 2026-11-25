use bevy::prelude::Message;
use derive_more::Deref;

use crate::turns;

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq, Deref)]
pub struct NextStep(pub turns::Step);

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq, Deref)]
pub struct StepStarted(pub turns::Step);

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct AllPlayersPassed;

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct MulligansComplete;
