#[path = "__basic__/mod.rs"]
mod basic;

pub use basic::card_text::{
    deathtouch, defender, double_strike, first_strike, flying, haste, hexproof, indestructible,
    lifelink, must_attack_if_able, must_block_if_able, reach, trample, vigilance,
};
pub use basic::enums::StaticKeyword;
