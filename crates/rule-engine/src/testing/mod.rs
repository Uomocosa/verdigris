#[path = "__app__/mod.rs"]
mod app;
#[path = "__fixtures__/mod.rs"]
mod fixtures;
#[path = "__app__/caster/__traits__/mod.rs"]
pub mod traits;

pub use app::{
    Card, Cards, CardsError, Cast, MockGame, MockGameBuilder, P1, P2, P3, P4, P5, P6, P7, P8,
    PlayError, PlayerSetup, PlayerView, Step, Targets,
};
pub use fixtures::{
    CANCEL, DARKSTEEL_MYR, DISPEL, DIVINATION, DOOM_BLADE, ESSENCE_SCATTER, ExpectedTargets,
    GIANT_GROWTH, GRIZZLY_BEARS, LIGHTNING_BOLT, MIND_ROT, MURDER, NEGATE, SIGN_IN_BLOOD,
    TYPHOID_RATS, WALKING_CORPSE, cancel, darksteel_myr, dispel, divination, doom_blade,
    essence_scatter, giant_growth, grizzly_bears, lightning_bolt, mind_rot, murder, negate, pick,
    sign_in_blood, typhoid_rats, walking_corpse,
};
