use crate::card;
use crate::cards;
use crate::cost;
use crate::events;
use crate::rules;
use crate::statics;
use crate::testing;

pub const TEST_CANT_DISCARD: testing::Card = testing::Card {
    name: testing::TestCardName("Test Cant Discard"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Cant Discard", cost![2]);
            statics::you().cant::<events::Discard>(rules::Role::Player),
            statics::you().cant::<events::GainLife>(rules::Role::Player),
            statics::you().cant::<events::Mill>(rules::Role::Player),
            statics::you().cant::<events::CreateTokens>(rules::Role::Player),
        }
    },
};
