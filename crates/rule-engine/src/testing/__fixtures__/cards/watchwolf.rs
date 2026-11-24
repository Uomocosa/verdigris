use crate::cards;
use crate::cost;
use crate::pt;
use crate::testing;

pub const WATCHWOLF: testing::Card = testing::Card {
    name: testing::TestCardName("Watchwolf"),
    definition: || cards::creature("Watchwolf", cost![G, W], pt!(3, 3)),
};
