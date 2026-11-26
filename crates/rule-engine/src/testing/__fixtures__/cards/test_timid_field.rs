use crate::CREATURE;
use crate::card;
use crate::cards;
use crate::cost;
use crate::events;
use crate::rules;
use crate::statics;
use crate::testing;

pub const TEST_TIMID_FIELD: testing::Card = testing::Card {
    name: testing::TestCardName("Test Timid Field"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Timid Field", cost![1, W]);
            statics::all(CREATURE)
                .restriction::<events::DeclareBlocker>(
                    events::DeclareBlocker::BLOCKER,
                    rules::RestrictionId("test/test-timid-field"),
                    "blocks only with enough power",
                    rules::at_least(rules::Characteristic::Power, 3)
                        .of(events::DeclareBlocker::BLOCKER),
                )
        }
    },
};
