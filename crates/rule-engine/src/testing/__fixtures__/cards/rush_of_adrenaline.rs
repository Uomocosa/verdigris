use crate::testing::prelude::*;

pub const RUSH_OF_ADRENALINE: Card = Card {
    name: TestCardName("Rush of Adrenaline"),
    definition: || {
        cards::instant("Rush of Adrenaline", cost![R])
            .spell(target(1, creature()).do_(get(pt!(+2, +1)) & gain(trample())))
    },
};
