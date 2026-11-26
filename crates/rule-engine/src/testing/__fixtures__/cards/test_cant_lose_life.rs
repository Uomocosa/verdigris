use crate::card;
use crate::cards;
use crate::cost;
use crate::events;
use crate::rules;
use crate::statics;
use crate::testing;

pub const TEST_CANT_LOSE_LIFE: testing::Card = testing::Card {
    name: testing::TestCardName("Test Can't Lose Life"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Can't Lose Life", cost![2]);
            statics::you().cant::<events::LoseLife>(
                events::LoseLife::PLAYER,
                rules::RestrictionId("test/test-cant-lose-life"),
            ),
        }
    },
};
