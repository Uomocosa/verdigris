use crate::testing;

pub const ALL_CARDS: &[testing::Card] = &[
    testing::ALWAYS_WATCHING,
    testing::AZUSA_LOST_BUT_SEEKING,
    testing::BLADEBRAND,
    testing::BORDERLAND_MARAUDER,
    testing::CANCEL,
    testing::CARESS_OF_PHYREXIA,
    testing::CHILD_OF_NIGHT,
    testing::CHUB_TOAD,
    testing::CRUCIBLE_OF_WORLDS,
    testing::DARKSTEEL_MYR,
    testing::DEMYSTIFY,
    testing::DISFIGURE,
    testing::DISPEL,
    testing::DIVINATION,
    testing::DOOM_BLADE,
    testing::ENDLESS_ONE,
    testing::ESSENCE_SCATTER,
    testing::FELIDAR_SOVEREIGN,
    testing::FENCING_ACE,
    testing::FOREST,
    testing::GARRUKS_COMPANION,
    testing::GIANT_GROWTH,
    testing::GIANT_SPIDER,
    testing::GLADECOVER_SCOUT,
    testing::GRIZZLY_BEARS,
    testing::GUT_SHOT,
    testing::HOLY_STRENGTH,
    testing::IRON_GOLEM,
    testing::ISAMARU_HOUND_OF_KONDA,
    testing::JUMP,
    testing::LABORATORY_MANIAC,
    testing::LEYLINE_OF_ANTICIPATION,
    testing::LEYLINE_OF_SANCTITY,
    testing::LEYLINE_OF_THE_VOID,
    testing::LIGHTNING_BOLT,
    testing::MASTER_OF_THE_FEAST,
    testing::MIND_ROT,
    testing::MOUNTAIN,
    testing::MURDER,
    testing::NEGATE,
    testing::PLAGUE_STINGER,
    testing::PLATINUM_ANGEL,
    testing::RAGING_GOBLIN,
    testing::RAISE_THE_ALARM,
    testing::SOLDIER_TOKEN,
    testing::RANGERS_GUILE,
    testing::RUSH_OF_ADRENALINE,
    testing::RELIQUARY_TOWER,
    testing::SCROLL_THIEF,
    testing::SIGN_IN_BLOOD,
    testing::SOUL_WARDEN,
    testing::SPHINX_OF_THE_SECOND_SUN,
    testing::SPORE_CRAWLER,
    testing::TEST_BATTLE_FERVOR,
    testing::TEST_CALL_TO_ARMS,
    testing::TEST_CANT_DISCARD,
    testing::TEST_CANT_LOSE_LIFE,
    testing::TEST_COVER_OF_DUSK,
    testing::TEST_DOUBLE_GIFT,
    testing::TEST_EAGER_WALL,
    testing::TEST_FIRST_STRIKE_GRANT,
    testing::TEST_FIRST_STRIKE_LORD,
    testing::TEST_FROZEN_RELIC,
    testing::TEST_GROUNDING_MIST,
    testing::TEST_HASTY_BANNER,
    testing::TEST_LOW_SKIMMER,
    testing::TEST_LURE_OF_BATTLE,
    testing::TEST_MAY_PURGE,
    testing::TEST_NEVER_BURIED,
    testing::TEST_NIGHT_OF_THE_HUNT,
    testing::TEST_NO_BURIALS,
    testing::TEST_NO_TOKENS,
    testing::TEST_OPENING_PURGE,
    testing::TEST_OPENING_TOLL,
    testing::TEST_PAINFUL_LESSONS,
    testing::TEST_PLAYER_RESTRICTION,
    testing::TEST_PLAYER_WAIVER,
    testing::TEST_RESTLESS_WATCHER,
    testing::TEST_RULE_ALLOWANCE,
    testing::TEST_RULE_RESTRICTION,
    testing::TEST_RULE_WAIVER,
    testing::TEST_SAFE_HAVEN,
    testing::TEST_SKY_COURT,
    testing::TEST_TAPPED_SENTRY,
    testing::TEST_TAUNTING_FIELD,
    testing::TEST_TIMID_FIELD,
    testing::TEST_WALL_OF_DREAD,
    testing::TEST_WAR_DRUMS,
    testing::TEST_WEIGHT_OF_COMMAND,
    testing::TOME_SCOUR,
    testing::TUNDRA,
    testing::TYPHOID_RATS,
    testing::UPWELLING,
    testing::WALKING_CORPSE,
    testing::WALL_OF_STONE,
    testing::WATCHWOLF,
    testing::KNIGHT_TOKEN,
    testing::WAYLAY,
    testing::WIND_DRAKE,
];

#[cfg(test)]
mod tests {
    use crate::testing;

    #[test]
    fn test_usage() {
        assert!(!testing::ALL_CARDS.is_empty());
        let bears = (testing::GRIZZLY_BEARS.definition)();
        assert_eq!(*bears.name, "Grizzly Bears");
        assert!(testing::ALL_CARDS.iter().any(|card| card.name == testing::GRIZZLY_BEARS.name));
    }

    #[test]
    fn all_cards_lists_every_fixture_card() {
        const SOURCE: &str = include_str!("mod.rs");
        let mut declared = 0;
        for line in SOURCE.lines() {
            if let Some(export) = line.strip_prefix("pub use ") {
                if export.starts_with("all_cards") {
                    continue;
                }
                declared += export.split(',').count();
            }
        }
        assert_eq!(
            testing::ALL_CARDS.len(),
            declared,
            "ALL_CARDS must list every fixture Card const"
        );
    }
}
