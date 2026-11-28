use crate::testing::prelude::*;

pub const DISFIGURE: Card = Card {
    name: TestCardName("Disfigure"),
    definition: || {
        cards::instant("Disfigure", cost![B]).spell(target(1, creature()).get(pt!(-2, -2)))
    },
};
