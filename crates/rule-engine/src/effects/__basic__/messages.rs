use bevy::prelude::{Entity, Message};

use crate::keywords;

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct PowerToughnessModified {
    pub target: Entity,
    pub power: i32,
    pub toughness: i32,
}

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct KeywordGranted {
    pub target: Entity,
    pub keyword: keywords::Keyword,
}
