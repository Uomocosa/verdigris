use crate::card;
use crate::cards;
use crate::cost;
use crate::creature;
use crate::keywords;
use crate::statics;
use crate::testing;

pub const HOLY_STRENGTH: testing::Card = testing::Card {
    name: testing::TestCardName("Holy Strength"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Holy Strength", cost![W])
                .subtype(cards::AURA);
            keywords::enchant(creature()),
            statics::enchanted().get_pt(1, 2),
        }
    },
};
