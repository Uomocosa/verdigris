#[path = "__basic__/mod.rs"]
mod basic;

pub use basic::card_text::{
    all, each_opponent, enchanted, enters_tapped, if_a, if_a_player, if_an_opponent, if_you, this,
    you,
};

#[path = "if.rs"]
mod if_;
pub use if_::If;

mod all;
pub use all::All;

mod would;
pub use would::Would;
