use crate::testing::prelude::*;

pub const BLADEBRAND: Card = Card {
    name: TestCardName("Bladebrand"),
    definition: || {
        cards::instant("Bladebrand", cost![1, B])
            .spell(target(1, creature()).gain(deathtouch()) & YOU.draw(1))
    },
};
