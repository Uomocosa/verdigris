use crate::CREATURE;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::keywords;
use crate::testing;

pub const JUMP: testing::Card = testing::Card {
    name: testing::TestCardName("Jump"),
    definition: || {
        cards::instant("Jump", cost![U])
            .spell(effects::gains(keywords::flying()).to(effects::target(1, CREATURE)))
    },
};
