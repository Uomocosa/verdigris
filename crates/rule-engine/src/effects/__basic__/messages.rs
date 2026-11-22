use bevy::prelude::{Entity, Message};

use crate::effects;
use crate::keywords;

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct PowerToughnessModified {
    pub target: Entity,
    pub power: i32,
    pub toughness: i32,
}

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct MillRequested {
    pub player: Entity,
    pub count: u32,
}

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokensRequested {
    pub count: u32,
    pub token: effects::TokenDefinition,
    pub controller: Entity,
}

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeywordGranted {
    pub target: Entity,
    pub keyword: keywords::StaticKeyword,
}
