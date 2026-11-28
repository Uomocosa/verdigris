use crate::testing::prelude::*;

pub const TEST_RESTLESS_WATCHER: Card = Card {
    name: TestCardName("Test Restless Watcher"),
    definition: || {
        card! {
            cards::creature("Test Restless Watcher", cost![2, U], pt!(2, 2));
            this().as_though(attack::UNTAPPED, Attack::ATTACKER),
        }
    },
};
