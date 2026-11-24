use crate::INSTANT_SPELL;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::testing;

pub const DISPEL: testing::Card = testing::Card {
    name: testing::TestCardName("Dispel"),
    definition: || {
        cards::instant("Dispel", cost![U])
            .spell(effects::counter().to(effects::target(1, INSTANT_SPELL)))
    },
};
