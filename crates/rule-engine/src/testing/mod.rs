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
    ALWAYS_WATCHING, CANCEL, CARESS_OF_PHYREXIA, CHILD_OF_NIGHT, DARKSTEEL_MYR, DISFIGURE, DISPEL,
    DIVINATION, DOOM_BLADE, ENDLESS_ONE, ESSENCE_SCATTER, ExpectedTargets, FELIDAR_SOVEREIGN,
    FENCING_ACE, FOREST, GARRUKS_COMPANION, GIANT_GROWTH, GIANT_SPIDER, GLADECOVER_SCOUT,
    GRIZZLY_BEARS, GUT_SHOT, HOLY_STRENGTH, IRON_GOLEM, ISAMARU_HOUND_OF_KONDA, JUMP, KNIGHT_TOKEN,
    LIGHTNING_BOLT, MASTER_OF_THE_FEAST, MIND_ROT, MOUNTAIN, MURDER, NEGATE, RAGING_GOBLIN,
    RAISE_THE_ALARM, SIGN_IN_BLOOD, SOLDIER_TOKEN, SOUL_WARDEN, SPORE_CRAWLER, TEST_BATTLE_FERVOR,
    TEST_CALL_TO_ARMS, TEST_COVER_OF_DUSK, TEST_GROUNDING_MIST, TEST_HASTY_BANNER,
    TEST_LURE_OF_BATTLE, TEST_NIGHT_OF_THE_HUNT, TEST_TAUNTING_FIELD, TEST_WALL_OF_DREAD,
    TEST_WAR_DRUMS, TEST_WEIGHT_OF_COMMAND, TOME_SCOUR, TUNDRA, TYPHOID_RATS, WALKING_CORPSE,
    WALL_OF_STONE, WATCHWOLF, WAYLAY, WIND_DRAKE, pick,
};

pub(crate) use app::{ActiveAutoPass, PassBudget};
