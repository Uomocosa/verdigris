use crate::CREATURE;
use crate::PLAYER;
use crate::YOU;
use crate::card;
use crate::cards;
use crate::cost;
use crate::rules;
use crate::statics;
use crate::testing;

pub const TEST_SAFE_HAVEN: testing::Card = testing::Card {
    name: testing::TestCardName("Test Safe Haven"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Safe Haven", cost![2]);
            statics::all(CREATURE.controlled_by(YOU)).cant::<rules::Destroy>(rules::Role::Subject),
            statics::all(PLAYER.controlled_by(YOU)).cant::<rules::Counter>(rules::Role::Player),
            statics::all(CREATURE.controlled_by(YOU)).cant::<rules::DealDamage>(rules::Role::Subject),
        }
    },
};
