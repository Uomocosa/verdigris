use crate::cards;
use crate::cost;
use crate::effects;
use crate::noncreature_spell;
use crate::testing;

pub const NEGATE: testing::Card = testing::Card {
    name: testing::TestCardName("Negate"),
    definition: || {
        cards::instant("Negate", cost![1, U])
            .spell(effects::counter(effects::target(1, noncreature_spell())))
    },
};
