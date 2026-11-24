use crate::cards;
use crate::cost;
use crate::effects;
use crate::testing;

pub const DIVINATION: testing::Card = testing::Card {
    name: testing::TestCardName("Divination"),
    definition: || cards::sorcery("Divination", cost![2, U]).spell(effects::draw(2)),
};
