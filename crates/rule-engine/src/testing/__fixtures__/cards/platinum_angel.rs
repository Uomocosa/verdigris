use crate::OPPONENT;
use crate::PLAYER;
use crate::YOU;
use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::pt;
use crate::statics;
use crate::testing;

pub const PLATINUM_ANGEL: testing::Card = testing::Card {
    name: testing::TestCardName("Platinum Angel"),
    definition: || {
        card! {
            cards::artifact_creature("Platinum Angel", cost![7], pt!(4, 4))
                .subtype(cards::CreatureType::Angel.into());
            keywords::flying(),
            statics::all(PLAYER.controlled_by(YOU)).cant_lose_the_game(),
            statics::all(PLAYER.controlled_by(OPPONENT)).cant_win_the_game(),
        }
    },
};
