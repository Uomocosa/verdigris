use crate::OPPONENT;
use crate::card;
use crate::cards;
use crate::cost;
use crate::creature;
use crate::effects;
use crate::statics;
use crate::testing;

pub const TEST_WEIGHT_OF_COMMAND: testing::Card = testing::Card {
    name: testing::TestCardName("Test Weight of Command"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Weight of Command", cost![2, U]);
            statics::all(creature() & effects::Filter::controlled_by(OPPONENT)).tap_when_attacking(),
        }
    },
};
