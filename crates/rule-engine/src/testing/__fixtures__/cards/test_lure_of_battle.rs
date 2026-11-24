use crate::CREATURE;
use crate::OPPONENT;
use crate::card;
use crate::cards;
use crate::cost;
use crate::statics;
use crate::testing;

pub const TEST_LURE_OF_BATTLE: testing::Card = testing::Card {
    name: testing::TestCardName("Test Lure of Battle"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Lure of Battle", cost![1, G]);
            statics::all(CREATURE.controlled_by(OPPONENT)).must_block(),
        }
    },
};
