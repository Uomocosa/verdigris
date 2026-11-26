use crate::effects;
use crate::events;
use crate::keywords;
use crate::rules;

/// 702.17b A creature with flying can’t be blocked except by creatures with flying and/or reach.
/// (See rule 509, “Declare Blockers Step,” and rule 702.9, “Flying.”)
///
/// # Reminder text
///
/// "This creature can block creatures with flying."
#[must_use]
pub fn r702_17b_reach() -> keywords::Keyword {
    keywords::Keyword::from_plain(ID, abilities)
}

pub const ID: keywords::KeywordId = keywords::KeywordId("702.17/reach");

fn abilities() -> Vec<effects::Modification> {
    vec![effects::Modification::Rule(rules::RuleChange {
        target: rules::Target::Restriction(keywords::flying::BLOCKERS),
        change: rules::Change::AsThough {
            holder: events::DeclareBlocker::BLOCKER.into(),
        },
    })]
}

#[cfg(test)]
mod tests {
    use crate::effects;
    use crate::events;
    use crate::keywords;
    use crate::rules;

    #[test]
    fn test_usage() {
        assert_eq!(super::r702_17b_reach().id, super::ID);
        assert_eq!(super::r702_17b_reach().abilities(), super::abilities());
        assert_eq!(
            super::abilities(),
            vec![effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::Restriction(keywords::flying::BLOCKERS),
                change: rules::Change::AsThough {
                    holder: events::DeclareBlocker::BLOCKER.into(),
                },
            })]
        );
    }
}
