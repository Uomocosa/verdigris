use crate::card;
use crate::cards;
use crate::cost;
use crate::events;
use crate::pt;
use crate::statics;
use crate::testing;

pub const TEST_RESTLESS_WATCHER: testing::Card = testing::Card {
    name: testing::TestCardName("Test Restless Watcher"),
    definition: || {
        card! {
            cards::creature("Test Restless Watcher", cost![2, U], pt!(2, 2));
            statics::this().as_though(events::declare_attacker::UNTAPPED),
        }
    },
};
