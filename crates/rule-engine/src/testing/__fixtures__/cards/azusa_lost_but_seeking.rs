use crate::testing::prelude::*;

pub const AZUSA_LOST_BUT_SEEKING: Card = Card {
    name: TestCardName("Azusa, Lost but Seeking"),
    definition: || {
        card! {
            cards::creature("Azusa, Lost but Seeking", cost![2, G], pt!(1, 2))
                .supertype(cards::Supertype::Legendary)
                .subtype(cards::HUMAN)
                .subtype(cards::MONK);
            you().play_additional_lands(2),
        }
    },
};
