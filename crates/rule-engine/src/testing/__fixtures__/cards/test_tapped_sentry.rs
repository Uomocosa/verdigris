use crate::card;
use crate::cards;
use crate::cost;
use crate::pt;
use crate::rules;
use crate::statics;
use crate::testing;

pub const TEST_TAPPED_SENTRY: testing::Card = testing::Card {
    name: testing::TestCardName("Test Tapped Sentry"),
    definition: || {
        card! {
            cards::creature("Test Tapped Sentry", cost![1, W], pt!(1, 4));
            statics::this().as_though(rules::declare_blocker::UNTAPPED),
        }
    },
};
