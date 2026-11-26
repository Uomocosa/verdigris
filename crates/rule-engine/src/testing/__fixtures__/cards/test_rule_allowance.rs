use crate::CREATURE;
use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::rules;
use crate::statics;
use crate::testing;

pub const TEST_RULE_ALLOWANCE: testing::Card = testing::Card {
    name: testing::TestCardName("Test Rule Allowance"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Rule Allowance", cost![2, U]);
            statics::all(CREATURE).allow(
                testing::UNTAPPED,
                rules::has_keyword(keywords::flying::ID).of(testing::TestRule::ATTACKER),
                testing::TestRule::ATTACKER,
            ),
        }
    },
};
