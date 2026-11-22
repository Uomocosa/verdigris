#[path = "__basic__/mod.rs"]
mod basic;

pub use basic::card_text::{all, enters_tapped, this};

mod all;
pub use all::All;
