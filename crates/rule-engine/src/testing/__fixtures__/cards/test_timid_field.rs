use crate::testing::prelude::*;

pub const TEST_TIMID_FIELD: Card = Card {
    name: TestCardName("Test Timid Field"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Timid Field", cost![1, W]);
            all(creature())
                .restriction::<DeclareBlocker>(
                    DeclareBlocker::BLOCKER,
                    RestrictionId("test/test-timid-field"),
                    power()
                        .of(DeclareBlocker::BLOCKER)
                        .is(Value(3).or_more()),
                )
        }
    },
};
