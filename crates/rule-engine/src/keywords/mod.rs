#[path = "__basic__/mod.rs"]
mod basic;

pub use basic::card_text::{
    deathtouch, defender, flying, haste, indestructible, lifelink, reach, trample, vigilance,
};
pub use basic::enums::Keyword;
