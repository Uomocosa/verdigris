use crate::testing::prelude::*;

pub const ISAMARU_HOUND_OF_KONDA: Card = Card {
    name: TestCardName("Isamaru, Hound of Konda"),
    definition: || {
        cards::creature("Isamaru, Hound of Konda", cost![W], pt!(2, 2))
            .supertype(cards::Supertype::Legendary)
    },
};
