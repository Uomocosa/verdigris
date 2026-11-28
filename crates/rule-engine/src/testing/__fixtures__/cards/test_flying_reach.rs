use crate::testing::prelude::*;

pub const TEST_FLYING_REACH: Card = Card {
    name: TestCardName("Test Flying Reach"),
    definition: || {
        card! {
            cards::creature("Test Flying Reach", cost![1, U], pt!(1, 1));
            flying(),
            reach(),
        }
    },
};
