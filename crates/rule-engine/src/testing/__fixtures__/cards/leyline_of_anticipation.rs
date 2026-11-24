use crate::PLAYER;
use crate::YOU;
use crate::abilities;
use crate::card;
use crate::cards;
use crate::cost;
use crate::statics;
use crate::testing;

pub const LEYLINE_OF_ANTICIPATION: testing::Card = testing::Card {
    name: testing::TestCardName("Leyline of Anticipation"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Leyline of Anticipation", cost![2, U, U]);
            abilities::begin_the_game_with_it_on_the_battlefield(),
            statics::all(PLAYER.controlled_by(YOU)).cast_spells_as_though_they_had_flash(),
        }
    },
};
