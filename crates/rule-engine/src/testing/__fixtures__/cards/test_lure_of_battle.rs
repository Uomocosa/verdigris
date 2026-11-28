use crate::testing::prelude::*;

pub const TEST_LURE_OF_BATTLE: Card = Card {
    name: TestCardName("Test Lure of Battle"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Lure of Battle", cost![1, G]);
            all(creature() & controlled_by(OPPONENT)).must_block(),
        }
    },
};
