use crate::effects;
use crate::keywords;
use crate::rules;

/// 702.2b A creature with toughness greater than 0 that’s been dealt damage by a source with deathtouch since the last time state-based actions were checked is destroyed as a state-based action. See rule 704.
pub struct Deathtouch;

impl Deathtouch {
    pub const ID: keywords::KeywordId = keywords::KeywordId("702.2/deathtouch");

    #[must_use]
    pub fn keyword() -> keywords::Keyword {
        keywords::Keyword::from_plain(Self::ID, Self::abilities)
    }

    #[must_use]
    pub fn abilities() -> Vec<effects::Modification> {
        vec![effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::value::<rules::LethalDamage>(),
            change: rules::Change::Set(1),
        })]
    }
}

#[cfg(test)]
mod tests {
    use super::Deathtouch;

    #[test]
    fn test_usage() {
        assert_eq!(Deathtouch::keyword().id, Deathtouch::ID);
        assert_eq!(Deathtouch::keyword().abilities(), Deathtouch::abilities());
    }
}
