use crate::CREATURE;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::pt;
use crate::testing;

pub const DISFIGURE: testing::Card = testing::Card {
    name: testing::TestCardName("Disfigure"),
    definition: || {
        cards::instant("Disfigure", cost![B]).spell(effects::target(1, CREATURE).get(pt!(-2, -2)))
    },
};
