use crate::testing::prelude::*;

pub const GLADECOVER_SCOUT: Card = Card {
    name: TestCardName("Gladecover Scout"),
    definition: || {
        card! {
            cards::creature("Gladecover Scout", cost![G], pt!(1, 1));
            hexproof(),
        }
    },
};
