use crate::NONBLACK_CREATURE;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::testing;

pub const DOOM_BLADE: testing::Card = testing::Card {
    name: testing::TestCardName("Doom Blade"),
    definition: || {
        cards::instant("Doom Blade", cost![1, B])
            .spell(effects::destroy().to(effects::target(1, NONBLACK_CREATURE)))
    },
};
