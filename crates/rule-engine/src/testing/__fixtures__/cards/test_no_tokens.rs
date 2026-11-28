use crate::testing::prelude::*;

pub const TEST_NO_TOKENS: Card = Card {
    name: TestCardName("Test No Tokens"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test No Tokens", cost![2]);
            you()
                .cant::<EnterTheBattlefield>(
                    EnterTheBattlefield::PERMANENT.controller(),
                    RestrictionId("test/test-no-tokens"),
                ),
        }
    },
};
