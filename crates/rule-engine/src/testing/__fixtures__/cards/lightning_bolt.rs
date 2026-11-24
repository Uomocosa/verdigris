use crate::ANY_TARGET;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::testing;

pub const LIGHTNING_BOLT: testing::Card = testing::Card {
    name: testing::TestCardName("Lightning Bolt"),
    definition: || {
        cards::instant("Lightning Bolt", cost![R])
            .spell(effects::damage(3).to(effects::target(1, ANY_TARGET)))
    },
};
