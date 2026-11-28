use crate::testing::prelude::*;

pub const TEST_PLAYER_RESTRICTION: Card = Card {
    name: TestCardName("Test Player Restriction"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Player Restriction", cost![2]);
            you().restriction::<TestRule>(
                TestRule::PLAYER,
                never(),
            ),
        }
    },
};
