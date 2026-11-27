use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::pt;
use crate::rules;
use crate::statics;
use crate::testing;

pub const TEST_BLOODSCENT_HOUND: testing::Card = testing::Card {
    name: testing::TestCardName("Test Bloodscent Hound"),
    definition: || {
        card! {
            cards::creature("Test Bloodscent Hound", cost![1, R], pt!(2, 1));
            statics::this().gain(keywords::haste()).as_long_as(rules::any(rules::Players::Opponents, rules::life().is(rules::Value(10).or_less()))),
        }
    },
};
