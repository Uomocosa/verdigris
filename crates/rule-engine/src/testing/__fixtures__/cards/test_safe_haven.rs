use crate::testing::prelude::*;

pub const TEST_SAFE_HAVEN: Card = Card {
    name: TestCardName("Test Safe Haven"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Safe Haven", cost![2]);
            all(creature().and(controlled_by(YOU))).cant::<Destroy>(
                Destroy::PERMANENT,
                RestrictionId("test/test-safe-haven"),
            ),
            you().cant::<Counter>(
                Counter::SPELL.controller(),
                RestrictionId("test/test-safe-haven"),
            ),
            all(creature().and(controlled_by(YOU))).cant::<DealDamage>(
                DealDamage::RECIPIENT,
                RestrictionId("test/test-safe-haven"),
            ),
        }
    },
};
