use crate::SPELL;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::testing;

pub const CANCEL: testing::Card = testing::Card {
    name: testing::TestCardName("Cancel"),
    definition: || {
        cards::instant("Cancel", cost![1, U, U])
            .spell(effects::counter().to(effects::target(1, SPELL)))
    },
};
