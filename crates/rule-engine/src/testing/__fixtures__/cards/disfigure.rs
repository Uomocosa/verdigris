use crate::cards;
use crate::cost;
use crate::creature;
use crate::effects;
use crate::pt;
use crate::testing;

pub const DISFIGURE: testing::Card = testing::Card {
    name: testing::TestCardName("Disfigure"),
    definition: || {
        cards::instant("Disfigure", cost![B]).spell(effects::target(1, creature()).get(pt!(-2, -2)))
    },
};
