use crate::cards;
use crate::cost;
use crate::pt;
use crate::testing;

pub const ENDLESS_ONE: testing::Card = testing::Card {
    name: testing::TestCardName("Endless One"),
    definition: || cards::creature("Endless One", cost![X], pt!(0, 0)),
};
