use crate::NONCREATURE_SPELL;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::testing;

pub const NEGATE: testing::Card = testing::Card {
    name: testing::TestCardName("Negate"),
    definition: || {
        cards::instant("Negate", cost![1, U])
            .spell(effects::counter().to(effects::target(1, NONCREATURE_SPELL)))
    },
};
