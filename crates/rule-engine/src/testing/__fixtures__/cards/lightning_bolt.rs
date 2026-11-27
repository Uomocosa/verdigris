use crate::THIS;
use crate::any_target;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::rules;
use crate::testing;

pub const LIGHTNING_BOLT: testing::Card = testing::Card {
    name: testing::TestCardName("Lightning Bolt"),
    definition: || {
        cards::instant("Lightning Bolt", cost![R])
            .spell(THIS.deals(rules::damage(3)).to(effects::target(1, any_target())))
    },
};
