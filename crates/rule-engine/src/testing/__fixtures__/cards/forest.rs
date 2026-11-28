use crate::testing::prelude::*;

pub const FOREST: Card = Card {
    name: TestCardName("Forest"),
    definition: || {
        cards::land("Forest", [cards::LandType::Forest]).supertype(cards::Supertype::Basic)
    },
};
