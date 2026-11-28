use crate::testing::prelude::*;

pub const DIVINATION: Card = Card {
    name: TestCardName("Divination"),
    definition: || cards::sorcery("Divination", cost![2, U]).spell(draw(2)),
};
