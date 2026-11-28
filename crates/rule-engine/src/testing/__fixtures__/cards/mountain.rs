use crate::testing::prelude::*;

pub const MOUNTAIN: Card = Card {
    name: TestCardName("Mountain"),
    definition: || {
        cards::land("Mountain", [cards::LandType::Mountain]).supertype(cards::Supertype::Basic)
    },
};
