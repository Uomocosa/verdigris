use crate::THIS;
use crate::YOU;
use crate::abilities;
use crate::card;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::pt;
use crate::rules;
use crate::testing;

pub const TEST_LAST_STAND: testing::Card = testing::Card {
    name: testing::TestCardName("Test Last Stand"),
    definition: || {
        card! {
            cards::creature("Test Last Stand", cost![2, B], pt!(3, 3));
            abilities::when(
                THIS.dies().if_(rules::power().of(THIS).is(rules::Value(3).or_more())),
                effects::draw(1).to_player(YOU),
            ),
        }
    },
};
