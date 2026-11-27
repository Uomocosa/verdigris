use crate::cards;
use crate::effects;
use crate::keywords;

pub fn enchant(definition: &cards::CardDefinition) -> Option<effects::Filter> {
    definition
        .keywords()
        .find_map(|keyword| keyword.params::<keywords::EnchantFilter>().map(effects::Filter::from))
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::cost;
    use crate::creature;
    use crate::keywords;

    use super::enchant;

    #[test]
    fn test_usage() {
        let definition = cards::CardDefinition::from_enchantment("Holy Strength", cost![W])
            .keyword(keywords::enchant(creature()));
        assert_eq!(enchant(&definition), Some(creature()));
    }
}
