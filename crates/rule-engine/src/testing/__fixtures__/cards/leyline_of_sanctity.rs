use crate::PLAYER;
use crate::YOU;
use crate::abilities;
use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::statics;
use crate::testing;

pub const LEYLINE_OF_SANCTITY: testing::Card = testing::Card {
    name: testing::TestCardName("Leyline of Sanctity"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Leyline of Sanctity", cost![2, W, W]);
            abilities::begin_the_game_with_it_on_the_battlefield(),
            statics::all(PLAYER.controlled_by(YOU)).gain(keywords::hexproof()),
        }
    },
};
