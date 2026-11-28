use crate::testing::prelude::*;

pub const CANCEL: Card = Card {
    name: TestCardName("Cancel"),
    definition: || cards::instant("Cancel", cost![1, U, U]).spell(counter(target(1, spell()))),
};
