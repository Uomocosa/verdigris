use crate::CREATURE;
use crate::YOU;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::keywords;
use crate::testing;

pub const RANGERS_GUILE: testing::Card = testing::Card {
    name: testing::TestCardName("Ranger's Guile"),
    definition: || {
        cards::instant("Ranger's Guile", cost![G]).spell(
            effects::gets(1, 1)
                .to(effects::target(1, CREATURE.controlled_by(YOU)))
                .and(effects::gains(keywords::hexproof())),
        )
    },
};
