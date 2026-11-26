use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::pt;
use crate::rules;
use crate::statics;
use crate::testing;

pub const TEST_LOW_SKIMMER: testing::Card = testing::Card {
    name: testing::TestCardName("Test Low Skimmer"),
    definition: || {
        card! {
            cards::creature("Test Low Skimmer", cost![1, U], pt!(1, 1));
            keywords::flying(),
            statics::this().as_though(keywords::Flying::FLYING_BLOCKERS, rules::Role::Other),
        }
    },
};
