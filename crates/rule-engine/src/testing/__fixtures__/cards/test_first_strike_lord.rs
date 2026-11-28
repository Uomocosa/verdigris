use crate::testing::prelude::*;

pub const TEST_FIRST_STRIKE_LORD: Card = Card {
    name: TestCardName("Test First Strike Lord"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test First Strike Lord", cost![1, W]);
            all(creature().and(controlled_by(YOU))).gain(first_strike()),
        }
    },
};
