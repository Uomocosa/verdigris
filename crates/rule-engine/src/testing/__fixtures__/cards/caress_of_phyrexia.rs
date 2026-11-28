use crate::counters;
use crate::testing::prelude::*;

pub const CARESS_OF_PHYREXIA: Card = Card {
    name: TestCardName("Caress of Phyrexia"),
    definition: || {
        cards::sorcery("Caress of Phyrexia", cost![3, B, B])
            .spell(target(1, PLAYER).do_(draw(3) & lose_life(3) & get(3 * counters::POISON)))
    },
};
