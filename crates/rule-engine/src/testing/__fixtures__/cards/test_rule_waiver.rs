use crate::card;
use crate::cards;
use crate::cost;
use crate::pt;
use crate::statics;
use crate::testing;

pub const TEST_RULE_WAIVER: testing::Card = testing::Card {
    name: testing::TestCardName("Test Rule Waiver"),
    definition: || {
        card! {
            cards::creature("Test Rule Waiver", cost![1], pt!(1, 1));
            statics::this().as_though(testing::UNTAPPED),
            statics::this().requirement::<testing::TestRule>(),
            statics::this().skip::<testing::TestStep>(),
        }
    },
};
