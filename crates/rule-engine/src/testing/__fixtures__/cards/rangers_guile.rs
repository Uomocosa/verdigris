use crate::YOU;
use crate::cards;
use crate::cost;
use crate::creature;
use crate::effects;
use crate::keywords;
use crate::pt;
use crate::testing;

pub const RANGERS_GUILE: testing::Card = testing::Card {
    name: testing::TestCardName("Ranger's Guile"),
    definition: || {
        cards::instant("Ranger's Guile", cost![G]).spell(
            effects::target(1, creature() & effects::Filter::controlled_by(YOU))
                .do_(effects::get(pt!(1, 1)) & effects::gain(keywords::hexproof())),
        )
    },
};
