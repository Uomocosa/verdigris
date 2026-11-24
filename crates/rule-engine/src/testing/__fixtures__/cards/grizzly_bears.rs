use crate::cards;
use crate::cost;
use crate::pt;
use crate::testing;

pub const GRIZZLY_BEARS: testing::Card = testing::Card {
    name: testing::TestCardName("Grizzly Bears"),
    definition: || cards::creature("Grizzly Bears", cost![1, G], pt!(2, 2)),
};
