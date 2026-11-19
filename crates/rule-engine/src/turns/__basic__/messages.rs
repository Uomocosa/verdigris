use bevy::prelude::Message;

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct CleanupStepStarted;
