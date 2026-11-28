use crate::testing::prelude::*;

pub const TEST_SKY_COURT: Card = Card {
    name: TestCardName("Test Sky Court"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Sky Court", cost![2, U]);
            all(creature()).restriction::<DeclareBlocker>(
                DeclareBlocker::BLOCKER,
                RestrictionId("test/test-sky-court"),
                !DeclareBlocker::BLOCKER.has(&flying()) | DeclareBlocker::ATTACKER.has(&flying()),
            ),
        }
    },
};
