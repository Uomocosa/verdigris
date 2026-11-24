use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::pt;
use crate::testing;

pub const PLAGUE_STINGER: testing::Card = testing::Card {
    name: testing::TestCardName("Plague Stinger"),
    definition: || {
        card! {
            cards::creature("Plague Stinger", cost![1, B], pt!(1, 1));
            keywords::flying(),
            keywords::infect(),
        }
    },
};
