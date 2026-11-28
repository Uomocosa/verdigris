use crate::testing::prelude::*;

pub const PLAGUE_STINGER: Card = Card {
    name: TestCardName("Plague Stinger"),
    definition: || {
        card! {
            cards::creature("Plague Stinger", cost![1, B], pt!(1, 1));
            flying(),
            infect(),
        }
    },
};
