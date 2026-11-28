use crate::testing::prelude::*;

pub const TEST_WALL_OF_DREAD: Card = Card {
    name: TestCardName("Test Wall of Dread"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Wall of Dread", cost![2, B]);
            all(creature().and(controlled_by(OPPONENT))).lose(defender()),
        }
    },
};
