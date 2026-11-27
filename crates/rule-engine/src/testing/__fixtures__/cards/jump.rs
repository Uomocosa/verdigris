use crate::cards;
use crate::cost;
use crate::creature;
use crate::effects;
use crate::keywords;
use crate::testing;

pub const JUMP: testing::Card = testing::Card {
    name: testing::TestCardName("Jump"),
    definition: || {
        cards::instant("Jump", cost![U])
            .spell(effects::target(1, creature()).gain(keywords::flying()))
    },
};
