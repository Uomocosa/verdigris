use crate::MAIN_2;
use crate::YOU;
use crate::abilities;
use crate::card;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::keywords;
use crate::pt;
use crate::testing;
use crate::turns;

pub const SPHINX_OF_THE_SECOND_SUN: testing::Card = testing::Card {
    name: testing::TestCardName("Sphinx of the Second Sun"),
    definition: || {
        card! {
            cards::creature("Sphinx of the Second Sun", cost![6, U, U], pt!(6, 6));
            keywords::flying(),
            abilities::at(
                turns::beginning_of(MAIN_2).of(YOU),
                effects::additional_phase(turns::Phase::Beginning),
            ),
        }
    },
};
