use crate::testing::prelude::*;

pub const TEST_CALL_TO_ARMS: Card = Card {
    name: TestCardName("Test Call to Arms"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Call to Arms", cost![1, R]);
            all(creature().and(controlled_by(OPPONENT))).must_attack(),
        }
    },
};
