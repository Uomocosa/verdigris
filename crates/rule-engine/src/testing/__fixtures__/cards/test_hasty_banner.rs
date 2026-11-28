use crate::testing::prelude::*;

pub const TEST_HASTY_BANNER: Card = Card {
    name: TestCardName("Test Hasty Banner"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Hasty Banner", cost![1, R]);
            all(creature().and(controlled_by(YOU))).gain(haste()),
        }
    },
};
