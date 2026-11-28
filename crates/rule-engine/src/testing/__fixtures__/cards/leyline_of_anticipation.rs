use crate::testing::prelude::*;

pub const LEYLINE_OF_ANTICIPATION: Card = Card {
    name: TestCardName("Leyline of Anticipation"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Leyline of Anticipation", cost![2, U, U]);
            begin_the_game_with_it_on_the_battlefield(),
            you().cast_spells_as_though_they_had_flash(),
        }
    },
};
