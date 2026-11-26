use crate::effects;
use crate::keywords;

/// 702.5a Enchant is a static ability, written “Enchant [object or player].” The enchant ability restricts what an Aura spell can target and what an Aura can enchant.
pub struct Enchant;

impl Enchant {
    pub const ID: keywords::KeywordId = keywords::KeywordId("702.5/enchant");

    #[must_use]
    pub fn keyword(filter: effects::Filter) -> keywords::Keyword {
        keywords::Keyword::from_with(Self::ID, keywords::EnchantFilter(filter), |_| Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::keywords;

    use super::Enchant;

    #[test]
    fn test_usage() {
        let keyword = Enchant::keyword(CREATURE);
        assert_eq!(keyword.id, Enchant::ID);
        assert_eq!(
            keyword.params::<keywords::EnchantFilter>(),
            Some(&keywords::EnchantFilter(CREATURE))
        );
        assert_eq!(keyword.abilities(), Vec::new());
    }
}
