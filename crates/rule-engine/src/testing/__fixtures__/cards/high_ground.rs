use crate::testing::prelude::*;

pub const HIGH_GROUND: Card = Card {
    name: TestCardName("High Ground"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("High Ground", cost![W]);
            all(creature() & controlled_by(YOU)).can_block_an_additional_creature(),
        }
    },
};
