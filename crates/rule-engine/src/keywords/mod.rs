#[path = "__basic__/mod.rs"]
mod basic;

pub use basic::card_text::{
    deathtouch, defender, double_strike, enchant, first_strike, flying, haste, hexproof,
    indestructible, infect, lifelink, reach, trample, vigilance,
};

mod static_keyword;
pub use static_keyword::StaticKeyword;
