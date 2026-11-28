use crate::testing::prelude::*;

pub const MURDER: Card = Card {
    name: TestCardName("Murder"),
    definition: || cards::instant("Murder", cost![1, B, B]).spell(destroy(target(1, creature()))),
};
