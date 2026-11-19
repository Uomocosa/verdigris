#[path = "__app__/mod.rs"]
mod app;
#[path = "__fixtures__/mod.rs"]
mod fixtures;

pub use app::{MockGame, MockGameBuilder};
pub use fixtures::{
    ExpectedTargets, Seat, cancel, darksteel_myr, divination, doom_blade, giant_growth,
    grizzly_bears, lightning_bolt, murder, pick, typhoid_rats, walking_corpse,
};
