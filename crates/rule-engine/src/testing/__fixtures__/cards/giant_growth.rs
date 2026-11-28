use crate::testing::prelude::*;

pub const GIANT_GROWTH: Card = Card {
    name: TestCardName("Giant Growth"),
    definition: || {
        cards::instant("Giant Growth", cost![G]).spell(target(1, creature()).get(pt!(3, 3)))
    },
};
