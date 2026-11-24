use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::pt;
use crate::testing;

pub const RAGING_GOBLIN: testing::Card = testing::Card {
    name: testing::TestCardName("Raging Goblin"),
    definition: || {
        card! {
            cards::creature("Raging Goblin", cost![R], pt!(1, 1));
            keywords::haste(),
        }
    },
};
