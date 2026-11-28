use crate::testing::prelude::*;

pub const TEST_GROUNDING_MIST: Card = Card {
    name: TestCardName("Test Grounding Mist"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Grounding Mist", cost![1, G]);
            all(creature().and(controlled_by(OPPONENT))).lose(flying()),
        }
    },
};
