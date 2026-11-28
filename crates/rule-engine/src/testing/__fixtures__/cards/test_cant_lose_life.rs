use crate::testing::prelude::*;

pub const TEST_CANT_LOSE_LIFE: Card = Card {
    name: TestCardName("Test Can't Lose Life"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Can't Lose Life", cost![2]);
            you().cant::<LoseLife>(
                LoseLife::PLAYER,
            ),
        }
    },
};
