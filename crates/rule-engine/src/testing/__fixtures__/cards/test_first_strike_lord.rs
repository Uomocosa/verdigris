use crate::CREATURE;
use crate::YOU;
use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::statics;
use crate::testing;

pub const TEST_FIRST_STRIKE_LORD: testing::Card = testing::Card {
    name: testing::TestCardName("Test First Strike Lord"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test First Strike Lord", cost![1, W]);
            statics::all(CREATURE.controlled_by(YOU)).gain(keywords::first_strike()),
        }
    },
};
