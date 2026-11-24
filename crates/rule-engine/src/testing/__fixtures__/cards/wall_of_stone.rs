use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::pt;
use crate::testing;

pub const WALL_OF_STONE: testing::Card = testing::Card {
    name: testing::TestCardName("Wall of Stone"),
    definition: || {
        card! {
            cards::creature("Wall of Stone", cost![1, R, R], pt!(0, 8));
            keywords::defender(),
        }
    },
};
