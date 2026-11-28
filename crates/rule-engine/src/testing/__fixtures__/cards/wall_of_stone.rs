use crate::testing::prelude::*;

pub const WALL_OF_STONE: Card = Card {
    name: TestCardName("Wall of Stone"),
    definition: || {
        card! {
            cards::creature("Wall of Stone", cost![1, R, R], pt!(0, 8));
            defender(),
        }
    },
};
