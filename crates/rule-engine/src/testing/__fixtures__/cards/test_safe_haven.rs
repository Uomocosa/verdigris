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
                rules::Role::Subject,
                rules::RestrictionId("test/test-safe-haven"),
                "can’t be destroyed",
            ),
            statics::you().cant::<events::Counter>(
                rules::Role::Player,
                rules::RestrictionId("test/test-safe-haven"),
                "can’t be countered",
            ),
            statics::all(CREATURE.controlled_by(YOU)).cant::<events::DealDamage>(
                rules::Role::Subject,
                rules::RestrictionId("test/test-safe-haven"),
                "can’t be dealt damage",
            ),
        }
    },
};
