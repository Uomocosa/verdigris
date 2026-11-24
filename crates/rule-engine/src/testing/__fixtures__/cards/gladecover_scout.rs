use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::pt;
use crate::testing;

pub const GLADECOVER_SCOUT: testing::Card = testing::Card {
    name: testing::TestCardName("Gladecover Scout"),
    definition: || {
        card! {
            cards::creature("Gladecover Scout", cost![G], pt!(1, 1));
            keywords::hexproof(),
        }
    },
};
