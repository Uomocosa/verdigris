use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::pt;
use crate::testing;

pub const DARKSTEEL_MYR: testing::Card = testing::Card {
    name: testing::TestCardName("Darksteel Myr"),
    definition: || {
        card! {
            cards::artifact_creature("Darksteel Myr", cost![3], pt!(0, 1));
            keywords::indestructible(),
        }
    },
};
