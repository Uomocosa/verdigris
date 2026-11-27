use crate::THIS;
use crate::abilities;
use crate::card;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::pt;
use crate::testing;

pub const BORDERLAND_MARAUDER: testing::Card = testing::Card {
    name: testing::TestCardName("Borderland Marauder"),
    definition: || {
        card! {
            cards::creature("Borderland Marauder", cost![1, R], pt!(1, 2));
            abilities::whenever(THIS.attacks(), effects::gets(2, 0).to(THIS)),
        }
    },
};
