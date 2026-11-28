use crate::testing::prelude::*;

pub const ESSENCE_SCATTER: Card = Card {
    name: TestCardName("Essence Scatter"),
    definition: || {
        cards::instant("Essence Scatter", cost![1, U]).spell(counter(target(1, creature_spell())))
    },
};
