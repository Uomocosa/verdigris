use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::pt;
use crate::testing;

pub const WIND_DRAKE: testing::Card = testing::Card {
    name: testing::TestCardName("Wind Drake"),
    definition: || {
        card! {
            cards::creature("Wind Drake", cost![2, U], pt!(2, 2));
            keywords::flying(),
        }
    },
};
