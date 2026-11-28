use crate::testing::prelude::*;

pub const ENDLESS_ONE: Card = Card {
    name: TestCardName("Endless One"),
    definition: || cards::creature("Endless One", cost![X], pt!(0, 0)),
};
