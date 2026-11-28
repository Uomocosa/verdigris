use crate::testing::prelude::*;

pub const HOLY_STRENGTH: Card = Card {
    name: TestCardName("Holy Strength"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Holy Strength", cost![W])
                .subtype(cards::AURA);
            enchant(creature()),
            enchanted().get_pt(1, 2),
        }
    },
};
