use crate::card;
use crate::cards;
use crate::cost;
use crate::events;
use crate::rules;
use crate::statics;
use crate::testing;

pub const TEST_NO_TOKENS: testing::Card = testing::Card {
    name: testing::TestCardName("Test No Tokens"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test No Tokens", cost![2]);
            statics::you()
                .cant::<events::EnterBattlefield>(
                    events::EnterBattlefield::CONTROLLER,
                    rules::RestrictionId("test/test-no-tokens"),
                    "can’t enter the battlefield",
                ),
        }
    },
};
