use crate::testing::prelude::*;

pub const DOOM_BLADE: Card = Card {
    name: TestCardName("Doom Blade"),
    definition: || {
        cards::instant("Doom Blade", cost![1, B]).spell(destroy(target(1, nonblack_creature())))
    },
};
