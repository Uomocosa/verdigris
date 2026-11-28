use crate::testing::prelude::*;

pub const DARKSTEEL_MYR: Card = Card {
    name: TestCardName("Darksteel Myr"),
    definition: || {
        card! {
            cards::artifact_creature("Darksteel Myr", cost![3], pt!(0, 1));
            indestructible(),
        }
    },
};
