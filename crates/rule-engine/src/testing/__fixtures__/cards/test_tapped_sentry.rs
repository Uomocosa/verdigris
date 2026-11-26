use crate::card;
use crate::cards;
use crate::cost;
use crate::events;
use crate::pt;
use crate::statics;
use crate::testing;

pub const TEST_TAPPED_SENTRY: testing::Card = testing::Card {
    name: testing::TestCardName("Test Tapped Sentry"),
    definition: || {
        card! {
            cards::creature("Test Tapped Sentry", cost![1, W], pt!(1, 4));
            statics::this().as_though(events::declare_blocker::UNTAPPED, events::DeclareBlocker::BLOCKER),
        }
    },
};
