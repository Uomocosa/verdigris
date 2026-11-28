use crate::testing::prelude::*;

pub const GARRUKS_COMPANION: Card = Card {
    name: TestCardName("Garruk's Companion"),
    definition: || {
        card! {
            cards::creature("Garruk's Companion", cost![G, G], pt!(3, 2));
            trample(),
        }
    },
};
