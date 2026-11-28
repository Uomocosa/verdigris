use crate::testing::prelude::*;

pub const TEST_WAR_DRUMS: Card = Card {
    name: TestCardName("Test War Drums"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test War Drums", cost![2, R]);
            all(creature() & controlled_by(YOU)).must_attack(),
        }
    },
};
