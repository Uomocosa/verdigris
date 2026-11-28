use crate::testing::prelude::*;

pub const TYPHOID_RATS: Card = Card {
    name: TestCardName("Typhoid Rats"),
    definition: || {
        card! {
            cards::creature("Typhoid Rats", cost![B], pt!(1, 1));
            deathtouch(),
        }
    },
};
