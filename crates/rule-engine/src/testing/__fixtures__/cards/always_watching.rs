use crate::testing::prelude::*;

pub const ALWAYS_WATCHING: Card = Card {
    name: TestCardName("Always Watching"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Always Watching", cost![1, W, W]);
            all(creature() & controlled_by(YOU) & !token()).get_pt(1, 1),
            all(creature() & controlled_by(YOU) & !token()).gain(vigilance()),
        }
    },
};
