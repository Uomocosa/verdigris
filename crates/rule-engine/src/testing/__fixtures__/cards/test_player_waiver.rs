use crate::card;
use crate::cards;
use crate::cost;
use crate::statics;
use crate::testing;

pub const TEST_PLAYER_WAIVER: testing::Card = testing::Card {
    name: testing::TestCardName("Test Player Waiver"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Player Waiver", cost![2, U]);
            statics::you().as_though(testing::UNTAPPED),
        }
    },
};
