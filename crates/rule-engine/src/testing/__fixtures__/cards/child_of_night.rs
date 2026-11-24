use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::pt;
use crate::testing;

pub const CHILD_OF_NIGHT: testing::Card = testing::Card {
    name: testing::TestCardName("Child of Night"),
    definition: || {
        card! {
            cards::creature("Child of Night", cost![1, B], pt!(2, 1));
            keywords::lifelink(),
        }
    },
};
