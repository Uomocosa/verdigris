use crate::testing::prelude::*;

pub const TEST_PLAYER_WAIVER: Card = Card {
    name: TestCardName("Test Player Waiver"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Player Waiver", cost![2, U]);
            you().as_though(UNTAPPED, TestRule::ATTACKER),
        }
    },
};
