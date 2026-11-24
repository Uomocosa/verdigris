use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::pt;
use crate::testing;

pub const GARRUKS_COMPANION: testing::Card = testing::Card {
    name: testing::TestCardName("Garruk's Companion"),
    definition: || {
        card! {
            cards::creature("Garruk's Companion", cost![G, G], pt!(3, 2));
            keywords::trample(),
        }
    },
};
