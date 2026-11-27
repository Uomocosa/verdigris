use crate::card;
use crate::cards;
use crate::cost;
use crate::creature;
use crate::events;
use crate::rules;
use crate::statics;
use crate::testing;

pub const TEST_TIMID_FIELD: testing::Card = testing::Card {
    name: testing::TestCardName("Test Timid Field"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Timid Field", cost![1, W]);
            statics::all(creature())
                .restriction::<events::DeclareBlocker>(
                    events::DeclareBlocker::BLOCKER,
                    rules::RestrictionId("test/test-timid-field"),
                    rules::power()
                        .of(events::DeclareBlocker::BLOCKER)
                        .is(rules::Value(3).or_more()),
                )
        }
    },
};
