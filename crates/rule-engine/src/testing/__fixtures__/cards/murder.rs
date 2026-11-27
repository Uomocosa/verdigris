use crate::CREATURE;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::testing;

pub const MURDER: testing::Card = testing::Card {
    name: testing::TestCardName("Murder"),
    definition: || {
        cards::instant("Murder", cost![1, B, B])
            .spell(effects::destroy(effects::target(1, CREATURE)))
    },
};
