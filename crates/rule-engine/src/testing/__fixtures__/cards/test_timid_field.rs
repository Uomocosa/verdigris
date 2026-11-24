use crate::CREATURE;
use crate::card;
use crate::cards;
use crate::cost;
use crate::rules;
use crate::statics;
use crate::testing;

pub const TEST_TIMID_FIELD: testing::Card = testing::Card {
    name: testing::TestCardName("Test Timid Field"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Timid Field", cost![1, W]);
            statics::all(CREATURE)
                .restriction::<rules::DeclareBlocker>(rules::at_least(
                    rules::Characteristic::Power,
                    3,
                )),
        }
    },
};
