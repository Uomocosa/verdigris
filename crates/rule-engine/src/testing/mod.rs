#[path = "__app__/mod.rs"]
mod app;
#[path = "__fixtures__/mod.rs"]
mod fixtures;

pub use app::{
    Card, Cards, CardsError, MockGame, MockGameBuilder, P1, P2, P3, P4, P5, P6, P7, P8, PlayError,
    PlayStep, PlayerSetup, PlayerView, Targets, TestPlayer, TestPlayerActions,
};
pub use fixtures::{
    CANCEL, DARKSTEEL_MYR, DISPEL, DIVINATION, DOOM_BLADE, ESSENCE_SCATTER, ExpectedTargets,
    GIANT_GROWTH, GRIZZLY_BEARS, LIGHTNING_BOLT, MIND_ROT, MURDER, NEGATE, SIGN_IN_BLOOD,
    TYPHOID_RATS, WALKING_CORPSE, pick,
};
