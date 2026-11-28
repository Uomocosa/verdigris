use crate::testing::prelude::*;

pub const TEST_TAPPED_SENTRY: Card = Card {
    name: TestCardName("Test Tapped Sentry"),
    definition: || {
        card! {
            cards::creature("Test Tapped Sentry", cost![1, W], pt!(1, 4));
            this().as_though(declare_blocker::UNTAPPED, DeclareBlocker::BLOCKER),
        }
    },
};
