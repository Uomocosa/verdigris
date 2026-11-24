use crate::cards;
use crate::testing;

pub const FOREST: testing::Card = testing::Card {
    name: testing::TestCardName("Forest"),
    definition: || {
        cards::land("Forest", [cards::LandType::Forest]).supertype(cards::Supertype::Basic)
    },
};
