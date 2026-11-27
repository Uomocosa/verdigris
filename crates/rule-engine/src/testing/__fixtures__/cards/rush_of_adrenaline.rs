use crate::CREATURE;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::keywords;
use crate::pt;
use crate::testing;

pub const RUSH_OF_ADRENALINE: testing::Card = testing::Card {
    name: testing::TestCardName("Rush of Adrenaline"),
    definition: || {
        cards::instant("Rush of Adrenaline", cost![R]).spell(
            effects::target(1, CREATURE)
                .do_(effects::get(pt!(+2, +1)) & effects::gain(keywords::trample())),
        )
    },
};
