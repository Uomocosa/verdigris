use crate::testing::prelude::*;

pub const SOUL_WARDEN: Card = Card {
    name: TestCardName("Soul Warden"),
    definition: || {
        card! {
            cards::creature("Soul Warden", cost![W], pt!(1, 1));
            whenever(another_creature().enters(), YOU.gain_life(1)),
        }
    },
};
