use crate::testing::prelude::*;

pub const TEST_WEIGHT_OF_COMMAND: Card = Card {
    name: TestCardName("Test Weight of Command"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Weight of Command", cost![2, U]);
            all(creature() & controlled_by(OPPONENT)).tap_when_attacking(),
        }
    },
};
