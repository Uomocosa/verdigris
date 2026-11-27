use crate::CREATURE;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::pt;
use crate::testing;

pub const GIANT_GROWTH: testing::Card = testing::Card {
    name: testing::TestCardName("Giant Growth"),
    definition: || {
        cards::instant("Giant Growth", cost![G]).spell(effects::target(1, CREATURE).gets(pt!(3, 3)))
    },
};
