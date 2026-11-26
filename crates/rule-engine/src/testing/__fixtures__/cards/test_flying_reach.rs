use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::pt;
use crate::testing;

pub const TEST_FLYING_REACH: testing::Card = testing::Card {
    name: testing::TestCardName("Test Flying Reach"),
    definition: || {
        card! {
            cards::creature("Test Flying Reach", cost![1, U], pt!(1, 1));
            keywords::flying(),
            keywords::reach(),
        }
    },
};
