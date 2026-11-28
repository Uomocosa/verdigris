use crate::testing::prelude::*;

pub const BORDERLAND_MARAUDER: Card = Card {
    name: TestCardName("Borderland Marauder"),
    definition: || {
        card! {
            cards::creature("Borderland Marauder", cost![1, R], pt!(1, 2));
            whenever(THIS.attacks(), THIS.get(pt!(+2, +0))),
        }
    },
};
