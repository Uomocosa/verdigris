use crate::testing::prelude::*;

pub const RAGING_GOBLIN: Card = Card {
    name: TestCardName("Raging Goblin"),
    definition: || {
        card! {
            cards::creature("Raging Goblin", cost![R], pt!(1, 1));
            haste(),
        }
    },
};
