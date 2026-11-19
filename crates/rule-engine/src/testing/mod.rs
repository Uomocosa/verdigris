#[path = "__app__/mod.rs"]
mod app;
#[path = "__fixtures__/mod.rs"]
mod fixtures;

pub use app::{MockGame, MockGameBuilder};
pub use fixtures::{
    ExpectedTargets, Seat, doom_blade, grizzly_bears, lightning_bolt, murder, pick, walking_corpse,
};
