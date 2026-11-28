use crate::testing::prelude::*;

pub const SERRA_ASCENDANT: Card = Card {
    name: TestCardName("Serra Ascendant"),
    definition: || {
        card! {
            cards::creature("Serra Ascendant", cost![W], pt!(1, 1));
            lifelink(),
            this().get_pt(5, 5).as_long_as(life().of(YOU).is(Value(30).or_more())),
            this().gain(flying()).as_long_as(life().of(YOU).is(Value(30).or_more())),
        }
    },
};
