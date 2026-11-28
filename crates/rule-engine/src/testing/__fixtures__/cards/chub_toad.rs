use crate::testing::prelude::*;

pub const CHUB_TOAD: Card = Card {
    name: TestCardName("Chub Toad"),
    definition: || {
        card! {
            cards::creature("Chub Toad", cost![2, G], pt!(1, 1));
            whenever(THIS.blocks(), THIS.get(pt!(3, 3))),
            whenever(THIS.becomes_blocked(), THIS.get(pt!(3, 3))),
        }
    },
};
