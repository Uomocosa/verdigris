use crate::CREATURE;
use crate::card;
use crate::cards;
use crate::cost;
use crate::events;
use crate::rules;
use crate::statics;
use crate::testing;

pub const TEST_SKY_COURT: testing::Card = testing::Card {
    name: testing::TestCardName("Test Sky Court"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Sky Court", cost![2, U]);
            statics::all(CREATURE).restriction::<events::DeclareBlocker>(
                rules::RestrictionId("test/test-sky-court"),
                "blocks only creatures with flying",
                !rules::has::<cards::Flying>() | rules::other(rules::has::<cards::Flying>()),
            ),
        }
    },
};
