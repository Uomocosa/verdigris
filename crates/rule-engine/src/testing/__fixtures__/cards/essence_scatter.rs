use crate::CREATURE_SPELL;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::testing;

pub const ESSENCE_SCATTER: testing::Card = testing::Card {
    name: testing::TestCardName("Essence Scatter"),
    definition: || {
        cards::instant("Essence Scatter", cost![1, U])
            .spell(effects::counter(effects::target(1, CREATURE_SPELL)))
    },
};
