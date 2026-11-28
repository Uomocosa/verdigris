use crate::testing::prelude::*;

pub const GRIZZLY_BEARS: Card = Card {
    name: TestCardName("Grizzly Bears"),
    definition: || cards::creature("Grizzly Bears", cost![1, G], pt!(2, 2)),
};
