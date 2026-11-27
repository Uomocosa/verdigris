use crate::THIS;
use crate::abilities;
use crate::card;
use crate::cards;
use crate::cost;
use crate::pt;
use crate::testing;

pub const CHUB_TOAD: testing::Card = testing::Card {
    name: testing::TestCardName("Chub Toad"),
    definition: || {
        card! {
            cards::creature("Chub Toad", cost![2, G], pt!(1, 1));
            abilities::whenever(THIS.blocks(), THIS.gets(pt!(3, 3))),
            abilities::whenever(THIS.becomes_blocked(), THIS.gets(pt!(3, 3))),
        }
    },
};
