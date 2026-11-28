use crate::testing::prelude::*;

pub const TUNDRA: Card = Card {
    name: TestCardName("Tundra"),
    definition: || cards::land("Tundra", [cards::LandType::Plains, cards::LandType::Island]),
};
