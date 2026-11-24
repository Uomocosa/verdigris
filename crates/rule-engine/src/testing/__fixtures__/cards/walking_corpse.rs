use crate::cards;
use crate::cost;
use crate::pt;
use crate::testing;

pub const WALKING_CORPSE: testing::Card = testing::Card {
    name: testing::TestCardName("Walking Corpse"),
    definition: || cards::creature("Walking Corpse", cost![1, B], pt!(2, 2)),
};
