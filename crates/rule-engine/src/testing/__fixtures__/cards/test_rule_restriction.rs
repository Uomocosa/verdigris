use crate::card;
use crate::cards;
use crate::cost;
use crate::pt;
use crate::rules;
use crate::statics;
use crate::testing;

pub const TEST_RULE_RESTRICTION: testing::Card = testing::Card {
    name: testing::TestCardName("Test Rule Restriction"),
    definition: || {
        card! {
            cards::creature("Test Rule Restriction", cost![2], pt!(1, 1));
            statics::this().restriction::<testing::RestrictedRule>(
                testing::RestrictedRule::ATTACKER,
                rules::RestrictionId("test/test-rule-restriction"),
                "attacks only with enough power",
                rules::at_least(rules::Characteristic::Power, 3)
                    .of(testing::RestrictedRule::ATTACKER),
            ),
        }
    },
};
