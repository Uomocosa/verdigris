use crate::testing::prelude::*;

pub const RANGERS_GUILE: Card = Card {
    name: TestCardName("Ranger's Guile"),
    definition: || {
        cards::instant("Ranger's Guile", cost![G]).spell(
            target(1, creature().and(controlled_by(YOU))).do_(get(pt!(1, 1)) & gain(hexproof())),
        )
    },
};
