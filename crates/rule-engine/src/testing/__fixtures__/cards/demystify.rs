use crate::ENCHANTMENT;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::testing;

pub const DEMYSTIFY: testing::Card = testing::Card {
    name: testing::TestCardName("Demystify"),
    definition: || {
        cards::instant("Demystify", cost![W])
            .spell(effects::destroy().to(effects::target(1, ENCHANTMENT)))
    },
};
