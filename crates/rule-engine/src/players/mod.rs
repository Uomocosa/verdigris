#[path = "__basic__/mod.rs"]
mod basic;

pub use basic::components::{
    AttemptedDrawFromEmptyLibrary, DefendingPlayer, GameIsADraw, LeftTheGame, Life, Lost, Player,
    Won,
};
pub use basic::status::{Status, StatusItem};
pub use basic::type_aliases::{LostOrGameIsADraw, StillInTheGame};
pub use vocabulary::players::PlayerId;

mod lands_played_this_turn;
pub use lands_played_this_turn::LandsPlayedThisTurn;

mod players;
pub use players::Players;
mod player_view;
pub use player_view::PlayerView;
