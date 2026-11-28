use crate::testing::prelude::*;

pub const LEYLINE_OF_SANCTITY: Card = Card {
    name: TestCardName("Leyline of Sanctity"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Leyline of Sanctity", cost![2, W, W]);
            begin_the_game_with_it_on_the_battlefield(),
            you().gain(hexproof()),
        }
    },
};
