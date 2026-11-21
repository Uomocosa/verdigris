use bevy::prelude::{Entity, Message};

use crate::cards;

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct PowerToughnessModified {
    pub target: Entity,
    pub power: i32,
    pub toughness: i32,
}

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeywordGranted {
    pub target: Entity,
    pub keyword: cards::Keyword,
}
