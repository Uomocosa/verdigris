use crate::testing::prelude::*;

pub const TEST_RULE_RESTRICTION: Card = Card {
    name: TestCardName("Test Rule Restriction"),
    definition: || {
        card! {
            cards::creature("Test Rule Restriction", cost![2], pt!(1, 1));
            this().restriction::<RestrictedRule>(
                RestrictedRule::ATTACKER,
                RestrictionId("test/test-rule-restriction"),
                power()
                    .of(RestrictedRule::ATTACKER)
                    .is(Value(3).or_more()),
            ),
        }
    },
};
