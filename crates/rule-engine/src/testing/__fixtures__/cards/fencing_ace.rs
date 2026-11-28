use crate::testing::prelude::*;

pub const FENCING_ACE: Card = Card {
    name: TestCardName("Fencing Ace"),
    definition: || {
        card! {
            cards::creature("Fencing Ace", cost![1, W], pt!(1, 1));
            double_strike(),
        }
    },
};
