use crate::testing::prelude::*;

pub const SPHINX_OF_THE_SECOND_SUN: Card = Card {
    name: TestCardName("Sphinx of the Second Sun"),
    definition: || {
        card! {
            cards::creature("Sphinx of the Second Sun", cost![6, U, U], pt!(6, 6));
            flying(),
            at(
                beginning_of(MAIN_2).of(YOU),
                additional_phase(Phase::Beginning),
            ),
        }
    },
};
