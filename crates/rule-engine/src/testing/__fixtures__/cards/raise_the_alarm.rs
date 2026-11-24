use crate::cards;
use crate::cost;
use crate::effects;
use crate::pt;
use crate::testing;

pub const SOLDIER_TOKEN: testing::Card = testing::Card {
    name: testing::TestCardName("Soldier"),
    definition: || cards::creature("Soldier", cost![W], pt!(1, 1)),
};

pub const RAISE_THE_ALARM: testing::Card = testing::Card {
    name: testing::TestCardName("Raise the Alarm"),
    definition: || {
        cards::instant("Raise the Alarm", cost![1, W])
            .spell(effects::creates(2, SOLDIER_TOKEN.definition.into()))
    },
};
