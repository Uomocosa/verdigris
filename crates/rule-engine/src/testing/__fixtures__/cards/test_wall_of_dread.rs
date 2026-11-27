use crate::OPPONENT;
use crate::card;
use crate::cards;
use crate::cost;
use crate::creature;
use crate::effects;
use crate::keywords;
use crate::statics;
use crate::testing;

pub const TEST_WALL_OF_DREAD: testing::Card = testing::Card {
    name: testing::TestCardName("Test Wall of Dread"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Wall of Dread", cost![2, B]);
            statics::all(creature() & effects::Filter::controlled_by(OPPONENT)).lose(keywords::defender()),
        }
    },
};
