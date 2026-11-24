use crate::CREATURE;
use crate::YOU;
use crate::card;
use crate::cards;
use crate::cost;
use crate::events;
use crate::rules;
use crate::statics;
use crate::testing;

pub const TEST_NO_BURIALS: testing::Card = testing::Card {
    name: testing::TestCardName("Test No Burials"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test No Burials", cost![2]);
            statics::all(CREATURE.controlled_by(YOU)).cant::<events::PutIntoGraveyard>(rules::Role::Subject),
        }
    },
};
