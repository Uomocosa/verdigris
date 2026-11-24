use crate::CREATURE;
use crate::YOU;
use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::statics;
use crate::testing;

pub const TEST_HASTY_BANNER: testing::Card = testing::Card {
    name: testing::TestCardName("Test Hasty Banner"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Hasty Banner", cost![1, R]);
            statics::all(CREATURE.controlled_by(YOU)).gain(keywords::haste()),
        }
    },
};
