use crate::testing::prelude::*;

pub const TEST_LOW_SKIMMER: Card = Card {
    name: TestCardName("Test Low Skimmer"),
    definition: || {
        card! {
            cards::creature("Test Low Skimmer", cost![1, U], pt!(1, 1));
            flying(),
            this().as_though(flying::BLOCKERS, DeclareBlocker::ATTACKER),
        }
    },
};
