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
                events::Discard::PLAYER,
                rules::RestrictionId("test/test-cant-discard"),
                "can’t discard",
            ),
            statics::you().cant::<events::GainLife>(
                events::GainLife::PLAYER,
                rules::RestrictionId("test/test-cant-discard"),
                "can’t gain life",
            ),
            statics::you().cant::<events::Mill>(
                events::Mill::PLAYER,
                rules::RestrictionId("test/test-cant-discard"),
                "can’t mill",
            ),
            statics::you().cant::<events::CreateTokens>(
                events::CreateTokens::CONTROLLER,
                rules::RestrictionId("test/test-cant-discard"),
                "can’t create tokens",
            ),
        }
    },
};
