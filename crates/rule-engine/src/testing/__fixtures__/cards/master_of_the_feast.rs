use crate::testing::prelude::*;

pub const MASTER_OF_THE_FEAST: Card = Card {
    name: TestCardName("Master of the Feast"),
    definition: || {
        card! {
            cards::creature("Master of the Feast", cost![4, B], pt!(5, 5));
            flying(),
            this().must_attack(),
            at(
                beginning_of(UPKEEP).of(YOU),
                each(OPPONENT).draw(1),
            ),
        }
    },
};
