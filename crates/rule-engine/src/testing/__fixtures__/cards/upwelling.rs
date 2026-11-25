use crate::PLAYER;
use crate::card;
use crate::cards;
use crate::cost;
use crate::statics;
use crate::testing;

pub const UPWELLING: testing::Card = testing::Card {
    name: testing::TestCardName("Upwelling"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Upwelling", cost![3, G]);
            statics::all(PLAYER).dont_lose_unspent_mana(),
        }
    },
};
