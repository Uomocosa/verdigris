#[path = "__basic__/mod.rs"]
mod basic;

pub use basic::components::{
    AttemptedDrawFromEmptyLibrary, DefendingPlayer, Drawn, Life, Lost, Player, PoisonCounters, Won,
};

mod lands_played_this_turn;
pub use lands_played_this_turn::LandsPlayedThisTurn;
