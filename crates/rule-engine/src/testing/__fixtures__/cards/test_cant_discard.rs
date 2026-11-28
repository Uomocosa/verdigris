use crate::testing::prelude::*;

pub const TEST_CANT_DISCARD: Card = Card {
    name: TestCardName("Test Cant Discard"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Cant Discard", cost![2]);
            you().cant::<Discard>(
                Discard::CARDS.owner(),
                RestrictionId("test/test-cant-discard"),
            ),
            you().cant::<GainLife>(
                GainLife::PLAYER,
                RestrictionId("test/test-cant-discard"),
            ),
            you().cant::<Mill>(
                Mill::PLAYER,
                RestrictionId("test/test-cant-discard"),
            ),
            you().cant::<Create>(
                Create::TOKEN.controller(),
                RestrictionId("test/test-cant-discard"),
            ),
        }
    },
};
