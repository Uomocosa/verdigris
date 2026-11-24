use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::pt;
use crate::testing;

pub const FENCING_ACE: testing::Card = testing::Card {
    name: testing::TestCardName("Fencing Ace"),
    definition: || {
        card! {
            cards::creature("Fencing Ace", cost![1, W], pt!(1, 1));
            keywords::double_strike(),
        }
    },
};
