use crate::CREATURE;
use crate::YOU;
use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::statics;
use crate::testing;

pub const ALWAYS_WATCHING: testing::Card = testing::Card {
    name: testing::TestCardName("Always Watching"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Always Watching", cost![1, W, W]);
            statics::all(CREATURE.controlled_by(YOU).nontoken()).get_pt(1, 1),
            statics::all(CREATURE.controlled_by(YOU).nontoken()).gain(keywords::vigilance()),
        }
    },
};
