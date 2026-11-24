use crate::CREATURE;
use crate::OPPONENT;
use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::statics;
use crate::testing;

pub const TEST_GROUNDING_MIST: testing::Card = testing::Card {
    name: testing::TestCardName("Test Grounding Mist"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Grounding Mist", cost![1, G]);
            statics::all(CREATURE.controlled_by(OPPONENT)).lose(keywords::flying()),
        }
    },
};
