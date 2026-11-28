use crate::testing::prelude::*;

pub const AMBUSH_VIPER: Card = Card {
    name: TestCardName("Ambush Viper"),
    definition: || {
        card! {
            cards::creature("Ambush Viper", cost![1, G], pt!(2, 1));
            flash(),
            deathtouch(),
        }
    },
};
