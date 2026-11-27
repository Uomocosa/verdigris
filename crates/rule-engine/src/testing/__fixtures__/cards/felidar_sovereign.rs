use crate::UPKEEP;
use crate::YOU;
use crate::abilities;
use crate::card;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::keywords;
use crate::pt;
use crate::rules;
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
                turns::beginning_of(UPKEEP).of(YOU).if_(rules::life().of(rules::YOU).is(rules::Value(40).or_more())),
                effects::win_the_game(),
            ),
        }
    },
};
