use crate::THIS;
use crate::abilities;
use crate::card;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::pt;
use crate::testing;

pub const CHUB_TOAD: testing::Card = testing::Card {
    name: testing::TestCardName("Chub Toad"),
    definition: || {
        card! {
            cards::creature("Chub Toad", cost![2, G], pt!(1, 1));
            abilities::whenever(
                abilities::TriggerEvent::blocks(),
                effects::gets(3, 3).to(THIS),
            ),
            abilities::whenever(
                abilities::TriggerEvent::becomes_blocked(),
                effects::gets(3, 3).to(THIS),
            ),
        }
    },
};
