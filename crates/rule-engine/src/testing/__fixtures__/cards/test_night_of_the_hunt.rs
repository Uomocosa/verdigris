use crate::testing::prelude::*;

pub const TEST_NIGHT_OF_THE_HUNT: Card = Card {
    name: TestCardName("Test Night of the Hunt"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Night of the Hunt", cost![2, B]);
            all(creature() & controlled_by(OPPONENT)).lose(vigilance()),
        }
    },
};
