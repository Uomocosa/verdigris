#[path = "__basic__/mod.rs"]
mod basic;

pub use basic::card_text::{all, each_opponent, enchanted, enters_tapped, this, you};

mod all;
pub use all::All;
