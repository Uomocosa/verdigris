use crate::YOU;
use crate::card;
use crate::cards;
use crate::cost;
use crate::creature;
use crate::effects;
use crate::keywords;
use crate::statics;
use crate::testing;

pub const ALWAYS_WATCHING: testing::Card = testing::Card {
    name: testing::TestCardName("Always Watching"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Always Watching", cost![1, W, W]);
            statics::all(creature() & effects::Filter::controlled_by(YOU) & !effects::Filter::token()).get_pt(1, 1),
            statics::all(creature() & effects::Filter::controlled_by(YOU) & !effects::Filter::token()).gain(keywords::vigilance()),
        }
    },
};
