use crate::testing::prelude::*;

pub const TEST_COVER_OF_DUSK: Card = Card {
    name: TestCardName("Test Cover of Dusk"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Cover of Dusk", cost![2, W]);
            all(creature() & controlled_by(YOU) & attacking()).attack_without_tapping(),
        }
    },
};
