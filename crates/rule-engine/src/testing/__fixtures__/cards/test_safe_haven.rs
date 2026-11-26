use crate::CREATURE;
use crate::YOU;
use crate::card;
use crate::cards;
use crate::cost;
use crate::events;
use crate::rules;
use crate::statics;
use crate::testing;

pub const TEST_SAFE_HAVEN: testing::Card = testing::Card {
    name: testing::TestCardName("Test Safe Haven"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Safe Haven", cost![2]);
            statics::all(CREATURE.controlled_by(YOU)).cant::<events::Destroy>(
                events::Destroy::PERMANENT,
                rules::RestrictionId("test/test-safe-haven"),
            ),
            statics::you().cant::<events::Counter>(
                events::Counter::SPELL.controller(),
                rules::RestrictionId("test/test-safe-haven"),
            ),
            statics::all(CREATURE.controlled_by(YOU)).cant::<events::DealDamage>(
                events::DealDamage::RECIPIENT,
                rules::RestrictionId("test/test-safe-haven"),
            ),
        }
    },
};
