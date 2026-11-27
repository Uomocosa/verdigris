use crate::YOU;
use crate::card;
use crate::cards;
use crate::cost;
use crate::creature;
use crate::effects;
use crate::keywords;
use crate::statics;
use crate::testing;

pub const TEST_HASTY_BANNER: testing::Card = testing::Card {
    name: testing::TestCardName("Test Hasty Banner"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Hasty Banner", cost![1, R]);
            statics::all(creature() & effects::Filter::controlled_by(YOU)).gain(keywords::haste()),
        }
    },
};
