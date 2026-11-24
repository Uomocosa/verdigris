use crate::PLAYER;
use crate::YOU;
use crate::card;
use crate::cards;
use crate::cost;
use crate::rules;
use crate::statics;
use crate::testing;

pub const TEST_CANT_DISCARD: testing::Card = testing::Card {
    name: testing::TestCardName("Test Cant Discard"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Cant Discard", cost![2]);
            statics::all(PLAYER.controlled_by(YOU)).cant::<rules::Discard>(rules::Role::Player),
            statics::all(PLAYER.controlled_by(YOU)).cant::<rules::GainLife>(rules::Role::Player),
            statics::all(PLAYER.controlled_by(YOU)).cant::<rules::Mill>(rules::Role::Player),
            statics::all(PLAYER.controlled_by(YOU)).cant::<rules::CreateTokens>(rules::Role::Player),
        }
    },
};
