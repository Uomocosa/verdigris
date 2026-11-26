use crate::effects;
use crate::events;
use crate::keywords;
use crate::rules;

/// 702.20a Vigilance is a static ability that modifies the rules for the declare attackers step.
pub struct Vigilance;

impl Vigilance {
    pub const ID: keywords::KeywordId = keywords::KeywordId("702.20/vigilance");

    #[must_use]
    pub fn keyword() -> keywords::Keyword {
        keywords::Keyword::from_plain(Self::ID, Self::abilities)
    }

    #[must_use]
    pub fn abilities() -> Vec<effects::Modification> {
        vec![effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::event::<events::TapAttacker>(),
            change: rules::Change::Skip,
        })]
    }
}

#[cfg(test)]
mod tests {
    use crate::effects;
    use crate::events;
    use crate::rules;

    use super::Vigilance;

    #[test]
    fn test_usage() {
        assert_eq!(Vigilance::keyword().id, Vigilance::ID);
        assert_eq!(Vigilance::keyword().abilities(), Vigilance::abilities());
        assert_eq!(
            Vigilance::abilities(),
            vec![effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::event::<events::TapAttacker>(),
                change: rules::Change::Skip,
            })]
        );
    }
}
