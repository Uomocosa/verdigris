use crate::testing::prelude::*;

pub const TEST_FIRST_STRIKE_GRANT: Card = Card {
    name: TestCardName("Test First Strike Grant"),
    definition: || {
        cards::instant("Test First Strike Grant", cost![R])
            .spell(target(1, creature()).gain(first_strike()))
    },
};
