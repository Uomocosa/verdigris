#[path = "__app__/mod.rs"]
mod app;
#[path = "__fixtures__/mod.rs"]
mod fixtures;

pub use app::{MockGame, MockGameBuilder, P1, P2, P3, P4, P5, P6, P7, P8, PlayerSetup};
pub use fixtures::{
    ExpectedTargets, Seat, cancel, darksteel_myr, dispel, divination, doom_blade, essence_scatter,
    giant_growth, grizzly_bears, lightning_bolt, mind_rot, murder, negate, pick, sign_in_blood,
    typhoid_rats, walking_corpse,
};
