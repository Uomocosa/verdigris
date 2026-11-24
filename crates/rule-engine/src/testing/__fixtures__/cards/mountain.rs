use crate::cards;
use crate::testing;

pub const MOUNTAIN: testing::Card = testing::Card {
    name: testing::TestCardName("Mountain"),
    definition: || {
        cards::land("Mountain", [cards::LandType::Mountain]).supertype(cards::Supertype::Basic)
    },
};
