use crate::GRAVEYARD;
use crate::PLAYER;
use crate::YOU;
use crate::card;
use crate::cards;
use crate::cost;
use crate::statics;
use crate::testing;

pub const CRUCIBLE_OF_WORLDS: testing::Card = testing::Card {
    name: testing::TestCardName("Crucible of Worlds"),
    definition: || {
        card! {
            cards::CardDefinition::from_artifact("Crucible of Worlds", cost![3]);
            statics::all(PLAYER.controlled_by(YOU)).play_lands_from(GRAVEYARD),
        }
    },
};
