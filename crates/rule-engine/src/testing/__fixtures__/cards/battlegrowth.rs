use crate::testing::prelude::*;

pub const BATTLEGROWTH: Card = Card {
    name: TestCardName("Battlegrowth"),
    definition: || {
        cards::instant("Battlegrowth", cost![G])
            .spell(target(1, creature()).get(1 * PLUS_ONE_PLUS_ONE))
    },
};
