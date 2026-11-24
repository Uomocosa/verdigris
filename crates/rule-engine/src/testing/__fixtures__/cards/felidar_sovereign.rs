use crate::UPKEEP;
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

pub const FELIDAR_SOVEREIGN: testing::Card = testing::Card {
    name: testing::TestCardName("Felidar Sovereign"),
    definition: || {
        card! {
            cards::creature("Felidar Sovereign", cost![4, W, W], pt!(4, 6));
            keywords::vigilance(),
            keywords::lifelink(),
            abilities::at(
                turns::beginning_of(UPKEEP, YOU).if_(YOU.has_life_at_least(40)),
                effects::win_the_game(),
            ),
        }
    },
};
