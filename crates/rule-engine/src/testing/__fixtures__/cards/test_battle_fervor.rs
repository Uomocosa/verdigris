use crate::testing::prelude::*;

pub const TEST_BATTLE_FERVOR: Card = Card {
    name: TestCardName("Test Battle Fervor"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Battle Fervor", cost![1, R]);
            all(creature().and(controlled_by(YOU))).attack_as_though_it_had_haste(),
        }
    },
};
