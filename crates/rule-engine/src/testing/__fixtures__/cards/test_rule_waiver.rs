use crate::testing::prelude::*;

pub const TEST_RULE_WAIVER: Card = Card {
    name: TestCardName("Test Rule Waiver"),
    definition: || {
        card! {
            cards::creature("Test Rule Waiver", cost![1], pt!(1, 1));
            this().as_though(UNTAPPED, TestRule::ATTACKER),
            this().requirement::<TestRule>(),
            this().skip::<TestStep>(),
        }
    },
};
