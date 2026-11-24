use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::pt;
use crate::statics;
use crate::testing;

pub const IRON_GOLEM: testing::Card = testing::Card {
    name: testing::TestCardName("Iron Golem"),
    definition: || {
        card! {
            cards::artifact_creature("Iron Golem", cost![4], pt!(5, 3));
            keywords::vigilance(),
            statics::this().must_attack(),
            statics::this().must_block(),
        }
    },
};
