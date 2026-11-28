use crate::testing::prelude::*;

pub const WALKING_CORPSE: Card = Card {
    name: TestCardName("Walking Corpse"),
    definition: || cards::creature("Walking Corpse", cost![1, B], pt!(2, 2)),
};
