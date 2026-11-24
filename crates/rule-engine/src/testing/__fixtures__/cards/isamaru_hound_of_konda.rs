use crate::cards;
use crate::cost;
use crate::pt;
use crate::testing;

pub const ISAMARU_HOUND_OF_KONDA: testing::Card = testing::Card {
    name: testing::TestCardName("Isamaru, Hound of Konda"),
    definition: || {
        cards::creature("Isamaru, Hound of Konda", cost![W], pt!(2, 2))
            .supertype(cards::Supertype::Legendary)
    },
};
