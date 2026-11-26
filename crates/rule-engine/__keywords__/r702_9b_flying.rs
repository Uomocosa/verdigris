use crate::cards;
use crate::effects;
use crate::events;
use crate::keywords;
use crate::rules;

/// 702.9b A creature with flying can’t be blocked except by creatures with flying and/or reach. A
/// creature with flying can block a creature with or without flying. (See rule 509, “Declare
/// Blockers Step,” and rule 702.17, “Reach.”)
#[must_use]
pub fn r702_9b_flying() -> keywords::Keyword {
    keywords::Keyword::from_plain(ID, abilities)
}

pub const ID: keywords::KeywordId = keywords::KeywordId("702.9/flying");
pub const BLOCKERS: rules::RestrictionId = rules::RestrictionId("702.9b/blockers");

fn abilities() -> Vec<effects::Modification> {
    vec![effects::Modification::Rule(rules::RuleChange {
        target: rules::Target::event::<events::DeclareBlocker>(),
        change: rules::Change::Restrict {
            holder: events::DeclareBlocker::ATTACKER.into(),
            restriction: BLOCKERS.restriction(
                (rules::has::<cards::Flying>() | rules::has::<cards::Reach>())
                    .of(events::DeclareBlocker::BLOCKER),
            ),
        },
    })]
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::effects;
    use crate::events;
    use crate::rules;

    #[test]
    fn test_usage() {
        assert_eq!(super::r702_9b_flying().id, super::ID);
        assert_eq!(super::r702_9b_flying().abilities(), super::abilities());
        assert_eq!(
            super::abilities(),
            vec![effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::event::<events::DeclareBlocker>(),
                change: rules::Change::Restrict {
                    holder: events::DeclareBlocker::ATTACKER.into(),
                    restriction: super::BLOCKERS.restriction(
                        (rules::has::<cards::Flying>() | rules::has::<cards::Reach>())
                            .of(events::DeclareBlocker::BLOCKER),
                    ),
                },
            })]
        );
    }
}
