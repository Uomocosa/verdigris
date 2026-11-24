#[path = "__basic__/mod.rs"]
mod basic;

pub use basic::components::{
    AttemptedDrawFromEmptyLibrary, DefendingPlayer, GameIsADraw, Life, Lost, Player,
    PoisonCounters, Won,
};
pub use basic::newtypes::PlayerId;
pub use basic::type_aliases::StillInTheGame;

mod lands_played_this_turn;
pub use lands_played_this_turn::LandsPlayedThisTurn;

mod players;
pub use players::Players;
mod player_view;
pub use player_view::PlayerView;
