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
            statics::you().cant::<events::Discard>(
                rules::Role::Player,
                rules::RestrictionId("test/test-cant-discard"),
                "can’t discard",
            ),
            statics::you().cant::<events::GainLife>(
                rules::Role::Player,
                rules::RestrictionId("test/test-cant-discard"),
                "can’t gain life",
            ),
            statics::you().cant::<events::Mill>(
                rules::Role::Player,
                rules::RestrictionId("test/test-cant-discard"),
                "can’t mill",
            ),
            statics::you().cant::<events::CreateTokens>(
                rules::Role::Player,
                rules::RestrictionId("test/test-cant-discard"),
                "can’t create tokens",
            ),
        }
    },
};
