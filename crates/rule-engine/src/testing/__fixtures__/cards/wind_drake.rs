use crate::testing::prelude::*;

pub const WIND_DRAKE: Card = Card {
    name: TestCardName("Wind Drake"),
    definition: || {
        card! {
            cards::creature("Wind Drake", cost![2, U], pt!(2, 2));
            flying(),
        }
    },
};
