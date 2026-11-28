use crate::testing::prelude::*;

pub const NEGATE: Card = Card {
    name: TestCardName("Negate"),
    definition: || {
        cards::instant("Negate", cost![1, U]).spell(counter(target(1, noncreature_spell())))
    },
};
