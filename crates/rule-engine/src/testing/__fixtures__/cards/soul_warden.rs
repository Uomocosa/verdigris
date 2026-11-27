use crate::ANOTHER_CREATURE;
use crate::YOU;
use crate::abilities;
use crate::card;
use crate::cards;
use crate::cost;
use crate::pt;
use crate::testing;

pub const SOUL_WARDEN: testing::Card = testing::Card {
    name: testing::TestCardName("Soul Warden"),
    definition: || {
        card! {
            cards::creature("Soul Warden", cost![W], pt!(1, 1));
            abilities::whenever(ANOTHER_CREATURE.enters(), YOU.gains_life(1)),
        }
    },
};
