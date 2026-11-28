use crate::testing::prelude::*;

pub const DISPEL: Card = Card {
    name: TestCardName("Dispel"),
    definition: || cards::instant("Dispel", cost![U]).spell(counter(target(1, instant_spell()))),
};
