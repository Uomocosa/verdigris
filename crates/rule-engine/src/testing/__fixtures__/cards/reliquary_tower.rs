use crate::abilities;
use crate::card;
use crate::cards;
use crate::statics;
use crate::testing;

pub const RELIQUARY_TOWER: testing::Card = testing::Card {
    name: testing::TestCardName("Reliquary Tower"),
    definition: || {
        card! {
            cards::land("Reliquary Tower", []);
            abilities::tap_add(cards::ManaType::Colorless),
            statics::you().no_maximum_hand_size(),
        }
    },
};
