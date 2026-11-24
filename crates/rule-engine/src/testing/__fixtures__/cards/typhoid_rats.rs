use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::pt;
use crate::testing;

pub const TYPHOID_RATS: testing::Card = testing::Card {
    name: testing::TestCardName("Typhoid Rats"),
    definition: || {
        card! {
            cards::creature("Typhoid Rats", cost![B], pt!(1, 1));
            keywords::deathtouch(),
        }
    },
};
