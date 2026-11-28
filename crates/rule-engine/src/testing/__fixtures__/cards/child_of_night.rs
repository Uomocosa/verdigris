use crate::testing::prelude::*;

pub const CHILD_OF_NIGHT: Card = Card {
    name: TestCardName("Child of Night"),
    definition: || {
        card! {
            cards::creature("Child of Night", cost![1, B], pt!(2, 1));
            lifelink(),
        }
    },
};
