use crate::testing::prelude::*;

pub const TEST_SAFE_HAVEN: Card = Card {
    name: TestCardName("Test Safe Haven"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Safe Haven", cost![2]);
            all(creature() & controlled_by(YOU)).cant::<Destroy>(
                Destroy::PERMANENT,
            ),
            you().cant::<Counter>(
                Counter::SPELL.controller(),
            ),
            all(creature() & controlled_by(YOU)).cant::<DealDamage>(
                DealDamage::RECIPIENT,
            ),
        }
    },
};
