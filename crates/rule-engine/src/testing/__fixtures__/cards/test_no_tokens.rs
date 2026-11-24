use crate::PLAYER;
use crate::YOU;
use crate::card;
use crate::cards;
use crate::cost;
use crate::rules;
use crate::statics;
use crate::testing;

pub const TEST_NO_TOKENS: testing::Card = testing::Card {
    name: testing::TestCardName("Test No Tokens"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test No Tokens", cost![2]);
            statics::all(PLAYER.controlled_by(YOU))
                .cant::<rules::EnterBattlefield>(rules::Role::Player),
        }
    },
};
