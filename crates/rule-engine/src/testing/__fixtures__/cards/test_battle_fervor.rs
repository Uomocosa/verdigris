use crate::YOU;
use crate::card;
use crate::cards;
use crate::cost;
use crate::creature;
use crate::effects;
use crate::statics;
use crate::testing;

pub const TEST_BATTLE_FERVOR: testing::Card = testing::Card {
    name: testing::TestCardName("Test Battle Fervor"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Battle Fervor", cost![1, R]);
            statics::all(creature() & effects::Filter::controlled_by(YOU)).attack_as_though_it_had_haste(),
        }
    },
};
