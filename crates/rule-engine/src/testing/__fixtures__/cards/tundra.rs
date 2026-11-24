use crate::cards;
use crate::testing;

pub const TUNDRA: testing::Card = testing::Card {
    name: testing::TestCardName("Tundra"),
    definition: || cards::land("Tundra", [cards::LandType::Plains, cards::LandType::Island]),
};
