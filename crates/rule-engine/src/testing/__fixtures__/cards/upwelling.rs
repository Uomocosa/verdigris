use crate::testing::prelude::*;

pub const UPWELLING: Card = Card {
    name: TestCardName("Upwelling"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Upwelling", cost![3, G]);
            all(PLAYER).dont_lose_unspent_mana(),
        }
    },
};
