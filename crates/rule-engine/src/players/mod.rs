#[path = "__basic__/mod.rs"]
mod basic;

pub use basic::components::{AttemptedDrawFromEmptyLibrary, Life, Lost, Player, Won};
pub use basic::messages::{DiscardRequested, DrawRequested, LifeLost};

mod lands_played_this_turn;
pub use lands_played_this_turn::LandsPlayedThisTurn;
