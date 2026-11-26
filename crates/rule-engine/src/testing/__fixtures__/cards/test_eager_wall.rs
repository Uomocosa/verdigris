use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::pt;
use crate::rules;
use crate::statics;
use crate::testing;

pub const TEST_EAGER_WALL: testing::Card = testing::Card {
    name: testing::TestCardName("Test Eager Wall"),
    definition: || {
        card! {
            cards::creature("Test Eager Wall", cost![1, W], pt!(0, 4));
            keywords::defender(),
            statics::this().as_though(keywords::Defender::CANT_ATTACK, rules::Role::Subject),
        }
    },
};
