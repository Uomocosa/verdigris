#[path = "__basic__/mod.rs"]
mod basic;

pub use basic::card_text::{
    deathtouch, defender, double_strike, first_strike, flying, haste, hexproof, indestructible,
    lifelink, reach, trample, vigilance,
};
pub use basic::enums::Keyword;
