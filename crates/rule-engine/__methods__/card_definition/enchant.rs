use crate::cards;
use crate::effects;
use crate::keywords;

pub fn enchant(definition: &cards::CardDefinition) -> Option<effects::Filter> {
    definition
        .keywords()
        .find_map(|keyword| keyword.params::<keywords::EnchantFilter>().map(|filter| **filter))
}

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::cards;
    use crate::cost;
    use crate::keywords;

    use super::enchant;

    #[test]
    fn test_usage() {
        let definition = cards::CardDefinition::from_enchantment("Holy Strength", cost![W])
            .keyword(keywords::enchant(CREATURE));
        assert_eq!(enchant(&definition), Some(CREATURE));
    }
}
