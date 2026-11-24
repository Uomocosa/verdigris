use crate::CREATURE;
use crate::card;
use crate::cards;
use crate::cost;
use crate::rules;
use crate::statics;
use crate::testing;

pub const TEST_SKY_COURT: testing::Card = testing::Card {
    name: testing::TestCardName("Test Sky Court"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Sky Court", cost![2, U]);
            statics::all(CREATURE).restriction::<rules::DeclareBlocker>(
                !rules::has::<cards::Flying>() | rules::other(rules::has::<cards::Flying>()),
            ),
        }
    },
};
