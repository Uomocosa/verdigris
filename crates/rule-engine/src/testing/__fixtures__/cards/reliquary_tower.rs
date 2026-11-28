use crate::testing::prelude::*;

pub const RELIQUARY_TOWER: Card = Card {
    name: TestCardName("Reliquary Tower"),
    definition: || {
        card! {
            cards::land("Reliquary Tower", []);
            tap_add(cards::ManaType::Colorless),
            you().no_maximum_hand_size(),
        }
    },
};
