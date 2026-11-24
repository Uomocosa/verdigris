use crate::CREATURE;
use crate::OPPONENT;
use crate::card;
use crate::cards;
use crate::cost;
use crate::statics;
use crate::testing;

pub const TEST_TAUNTING_FIELD: testing::Card = testing::Card {
    name: testing::TestCardName("Test Taunting Field"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Taunting Field", cost![1, G]);
            statics::all(CREATURE.controlled_by(OPPONENT)).must_block(),
        }
    },
};
