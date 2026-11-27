use crate::OPPONENT;
use crate::card;
use crate::cards;
use crate::cost;
use crate::creature;
use crate::effects;
use crate::keywords;
use crate::statics;
use crate::testing;

pub const TEST_NIGHT_OF_THE_HUNT: testing::Card = testing::Card {
    name: testing::TestCardName("Test Night of the Hunt"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Night of the Hunt", cost![2, B]);
            statics::all(creature() & effects::Filter::controlled_by(OPPONENT)).lose(keywords::vigilance()),
        }
    },
};
