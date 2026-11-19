#[path = "__app__/mod.rs"]
mod app;
#[path = "__fixtures__/mod.rs"]
mod fixtures;

pub use app::{MockGame, MockGameBuilder};
pub use fixtures::{
    ExpectedTargets, Seat, cancel, darksteel_myr, dispel, divination, doom_blade, essence_scatter,
    giant_growth, grizzly_bears, lightning_bolt, murder, negate, pick, typhoid_rats,
    walking_corpse,
};
