use crate::cards;
use crate::cost;
use crate::effects;
use crate::enchantment;
use crate::testing;

pub const DEMYSTIFY: testing::Card = testing::Card {
    name: testing::TestCardName("Demystify"),
    definition: || {
        cards::instant("Demystify", cost![W])
            .spell(effects::destroy(effects::target(1, enchantment())))
    },
};
