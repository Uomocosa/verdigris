use crate::cards;
use crate::cost;
use crate::effects;
use crate::instant_spell;
use crate::testing;

pub const DISPEL: testing::Card = testing::Card {
    name: testing::TestCardName("Dispel"),
    definition: || {
        cards::instant("Dispel", cost![U])
            .spell(effects::counter(effects::target(1, instant_spell())))
    },
};
