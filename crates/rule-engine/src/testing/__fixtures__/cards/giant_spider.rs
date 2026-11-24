use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::pt;
use crate::testing;

pub const GIANT_SPIDER: testing::Card = testing::Card {
    name: testing::TestCardName("Giant Spider"),
    definition: || {
        card! {
            cards::creature("Giant Spider", cost![3, G], pt!(2, 4));
            keywords::reach(),
        }
    },
};
