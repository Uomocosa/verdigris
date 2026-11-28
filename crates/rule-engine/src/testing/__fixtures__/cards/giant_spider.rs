use crate::testing::prelude::*;

pub const GIANT_SPIDER: Card = Card {
    name: TestCardName("Giant Spider"),
    definition: || {
        card! {
            cards::creature("Giant Spider", cost![3, G], pt!(2, 4));
            reach(),
        }
    },
};
