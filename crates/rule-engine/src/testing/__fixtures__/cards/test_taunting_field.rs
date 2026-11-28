use crate::testing::prelude::*;

pub const TEST_TAUNTING_FIELD: Card = Card {
    name: TestCardName("Test Taunting Field"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Taunting Field", cost![1, G]);
            all(creature().and(controlled_by(OPPONENT))).must_block(),
        }
    },
};
