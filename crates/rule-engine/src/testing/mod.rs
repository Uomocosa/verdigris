#[path = "__app__/mod.rs"]
mod app;
#[path = "__fixtures__/mod.rs"]
mod fixtures;

pub use app::{
    AutoPassUntil, Card, CardQuery, Cards, CardsError, CardsInZone, MockGame, MockGameBuilder, P1,
    P2, P3, P4, P5, P6, P7, P8, PlayError, PlayStep, PlayerSeat, PlayerSetup, PlayerView, Targets,
    TestAbility, TestCard, TestCardName, TestError, TestPlayer, TestPlayerActions, TestResult,
};
pub use fixtures::{
    CANCEL, DARKSTEEL_MYR, DISFIGURE, DISPEL, DIVINATION, DOOM_BLADE, ESSENCE_SCATTER,
    ExpectedTargets, FOREST, GIANT_GROWTH, GRIZZLY_BEARS, LIGHTNING_BOLT, MIND_ROT, MOUNTAIN,
    MURDER, NEGATE, SIGN_IN_BLOOD, TUNDRA, TYPHOID_RATS, WALKING_CORPSE, pick,
};

pub(crate) use app::{ActiveAutoPass, PassBudget};
