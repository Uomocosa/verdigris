use crate::THIS;
use crate::abilities;
use crate::card;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::pt;
use crate::testing;

pub const SPORE_CRAWLER: testing::Card = testing::Card {
    name: testing::TestCardName("Spore Crawler"),
    definition: || {
        card! {
            cards::creature("Spore Crawler", cost![2, G], pt!(3, 2));
            abilities::when(THIS.dies(), effects::draw(1)),
        }
    },
};
