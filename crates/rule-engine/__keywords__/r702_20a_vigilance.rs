use crate::effects;
use crate::events;
use crate::keywords;
use crate::rules;

/// 702.20a Vigilance is a static ability that modifies the rules for the declare attackers step.
#[must_use]
pub fn r702_20a_vigilance() -> keywords::Keyword {
    keywords::Keyword::from_plain(ID, abilities)
}

pub const ID: keywords::KeywordId = keywords::KeywordId("702.20/vigilance");

fn abilities() -> Vec<effects::Modification> {
    vec![effects::Modification::Rule(rules::RuleChange {
        target: rules::Target::event::<events::TapAttacker>(),
        change: rules::Change::Skip,
    })]
}

#[cfg(test)]
mod tests {
    use crate::effects;
    use crate::events;
    use crate::rules;

    #[test]
    fn test_usage() {
        assert_eq!(super::r702_20a_vigilance().id, super::ID);
        assert_eq!(super::r702_20a_vigilance().abilities(), super::abilities());
        assert_eq!(
            super::abilities(),
            vec![effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::event::<events::TapAttacker>(),
                change: rules::Change::Skip,
            })]
        );
    }
}
