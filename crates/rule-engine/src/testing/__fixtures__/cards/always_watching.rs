use crate::testing::prelude::*;

pub const ALWAYS_WATCHING: Card = Card {
    name: TestCardName("Always Watching"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Always Watching", cost![1, W, W]);
            all(creature().and(controlled_by(YOU)).and(not(token()))).get_pt(1, 1),
            all(creature().and(controlled_by(YOU)).and(not(token()))).gain(vigilance()),
        }
    },
};
