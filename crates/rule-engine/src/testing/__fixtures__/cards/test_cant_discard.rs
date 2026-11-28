use crate::testing::prelude::*;

pub const TEST_CANT_DISCARD: Card = Card {
    name: TestCardName("Test Cant Discard"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Cant Discard", cost![2]);
            you().cant::<Discard>(
                Discard::CARDS.owner(),
            ),
            you().cant::<GainLife>(
                GainLife::PLAYER,
            ),
            you().cant::<Mill>(
                Mill::PLAYER,
            ),
            you().cant::<Create>(
                Create::TOKEN.controller(),
            ),
        }
    },
};
