use crate::card;
use crate::cards;
use crate::cost;
use crate::pt;
use crate::statics;
use crate::testing;

pub const AZUSA_LOST_BUT_SEEKING: testing::Card = testing::Card {
    name: testing::TestCardName("Azusa, Lost but Seeking"),
    definition: || {
        card! {
            cards::creature("Azusa, Lost but Seeking", cost![2, G], pt!(1, 2))
                .supertype(cards::Supertype::Legendary)
                .subtype(cards::HUMAN)
                .subtype(cards::MONK);
            statics::you().play_additional_lands(2),
        }
    },
};
