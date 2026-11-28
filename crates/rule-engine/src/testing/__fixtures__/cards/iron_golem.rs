use crate::testing::prelude::*;

pub const IRON_GOLEM: Card = Card {
    name: TestCardName("Iron Golem"),
    definition: || {
        card! {
            cards::artifact_creature("Iron Golem", cost![4], pt!(5, 3));
            vigilance(),
            this().must_attack(),
            this().must_block(),
        }
    },
};
