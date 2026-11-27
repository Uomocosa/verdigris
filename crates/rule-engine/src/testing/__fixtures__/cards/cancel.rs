use crate::cards;
use crate::cost;
use crate::effects;
use crate::spell;
use crate::testing;

pub const CANCEL: testing::Card = testing::Card {
    name: testing::TestCardName("Cancel"),
    definition: || {
        cards::instant("Cancel", cost![1, U, U])
            .spell(effects::counter(effects::target(1, spell())))
    },
};
