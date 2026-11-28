use crate::testing::prelude::*;

pub const TEST_RULE_ALLOWANCE: Card = Card {
    name: TestCardName("Test Rule Allowance"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Rule Allowance", cost![2, U]);
            all(creature()).allow(
                UNTAPPED,
                has_keyword(flying::ID).of(TestRule::ATTACKER),
                TestRule::ATTACKER,
            ),
        }
    },
};
