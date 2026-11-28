use crate::testing::prelude::*;

pub const LIGHTNING_BOLT: Card = Card {
    name: TestCardName("Lightning Bolt"),
    definition: || {
        cards::instant("Lightning Bolt", cost![R])
            .spell(THIS.deals(damage(3)).to(target(1, any_target())))
    },
};
