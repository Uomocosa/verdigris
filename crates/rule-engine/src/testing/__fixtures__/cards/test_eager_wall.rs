use crate::testing::prelude::*;

pub const TEST_EAGER_WALL: Card = Card {
    name: TestCardName("Test Eager Wall"),
    definition: || {
        card! {
            cards::creature("Test Eager Wall", cost![1, W], pt!(0, 4));
            defender(),
            this().as_though(defender::CANT_ATTACK, Attack::ATTACKER),
        }
    },
};
