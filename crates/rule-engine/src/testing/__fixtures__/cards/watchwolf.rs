use crate::testing::prelude::*;

pub const WATCHWOLF: Card = Card {
    name: TestCardName("Watchwolf"),
    definition: || cards::creature("Watchwolf", cost![G, W], pt!(3, 3)),
};
