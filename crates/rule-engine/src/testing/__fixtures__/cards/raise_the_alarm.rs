use crate::testing::prelude::*;

pub const SOLDIER_TOKEN: Card = Card {
    name: TestCardName("Soldier"),
    definition: || cards::creature("Soldier", cost![W], pt!(1, 1)),
};

pub const RAISE_THE_ALARM: Card = Card {
    name: TestCardName("Raise the Alarm"),
    definition: || {
        cards::instant("Raise the Alarm", cost![1, W])
            .spell(creates(2, SOLDIER_TOKEN.definition.into()))
    },
};
