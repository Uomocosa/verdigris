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
                events::Discard::CARDS.owner(),
                rules::RestrictionId("test/test-cant-discard"),
            ),
            statics::you().cant::<events::GainLife>(
                events::GainLife::PLAYER,
                rules::RestrictionId("test/test-cant-discard"),
            ),
            statics::you().cant::<events::Mill>(
                events::Mill::PLAYER,
                rules::RestrictionId("test/test-cant-discard"),
            ),
            statics::you().cant::<events::Create>(
                events::Create::TOKEN.controller(),
                rules::RestrictionId("test/test-cant-discard"),
            ),
        }
    },
};
