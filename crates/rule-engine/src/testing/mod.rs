#[path = "__app__/mod.rs"]
mod app;
#[path = "__basic__/mod.rs"]
mod basic;
#[path = "__fixtures__/mod.rs"]
mod fixtures;

pub use app::{
    AutoPassUntil, Card, CardQuery, Cards, CardsError, CardsInZone, MockGame, MockGameBuilder, P1,
    P2, P3, P4, P5, P6, P7, P8, PlayError, PlayStep, PlayerSeat, PlayerSetup, PlayerView, Targets,
    TestAbility, TestCard, TestError, TestPlayer, TestPlayerActions, TestResult, TestTrigger,
    TriggerQuery, ZoneObject, ZoneOwner,
};
pub use basic::newtypes::TestCardName;
pub use fixtures::{
    CANCEL, CHILD_OF_NIGHT, DARKSTEEL_MYR, DISFIGURE, DISPEL, DIVINATION, DOOM_BLADE,
    ESSENCE_SCATTER, ExpectedTargets, FELIDAR_SOVEREIGN, FOREST, GARRUKS_COMPANION, GIANT_GROWTH,
    GIANT_SPIDER, GRIZZLY_BEARS, ISAMARU_HOUND_OF_KONDA, JUMP, LIGHTNING_BOLT, MIND_ROT, MOUNTAIN,
    MURDER, NEGATE, RAGING_GOBLIN, SIGN_IN_BLOOD, SOUL_WARDEN, TUNDRA, TYPHOID_RATS,
    WALKING_CORPSE, WALL_OF_STONE, WATCHWOLF, WIND_DRAKE, pick,
};

pub(crate) use app::{ActiveAutoPass, PassBudget};
