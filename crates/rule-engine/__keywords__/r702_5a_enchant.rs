use crate::effects;
use crate::keywords;

/// 702.5a Enchant is a static ability, written “Enchant [object or player].” The enchant ability
/// restricts what an Aura spell can target and what an Aura can enchant.
#[must_use]
pub fn r702_5a_enchant(filter: effects::Filter) -> keywords::Keyword {
    keywords::Keyword::from_with(ID, keywords::EnchantFilter(filter), |_| Vec::new())
}

pub const ID: keywords::KeywordId = keywords::KeywordId("702.5/enchant");

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::keywords;

    #[test]
    fn test_usage() {
        let keyword = super::r702_5a_enchant(CREATURE);
        assert_eq!(keyword.id, super::ID);
        assert_eq!(
            keyword.params::<keywords::EnchantFilter>(),
            Some(&keywords::EnchantFilter(CREATURE))
        );
        assert_eq!(keyword.abilities(), Vec::new());
    }
}
