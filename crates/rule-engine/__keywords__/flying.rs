use crate::cards;
use crate::effects;
use crate::events;
use crate::keywords;
use crate::rules;

/// 702.9b A creature with flying can’t be blocked except by creatures with flying and/or reach. A creature with flying can block a creature with or without flying. (See rule 509, “Declare Blockers Step,” and rule 702.17, “Reach.”)
pub struct Flying;

impl Flying {
    pub const ID: keywords::KeywordId = keywords::KeywordId("702.9/flying");
    pub const FLYING_BLOCKERS: rules::RestrictionId = rules::RestrictionId("702.9b/blockers");

    #[must_use]
    pub fn keyword() -> keywords::Keyword {
        keywords::Keyword::from_plain(Self::ID, Self::abilities)
    }

    #[must_use]
    pub fn abilities() -> Vec<effects::Modification> {
        vec![effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::event::<events::DeclareBlocker>(),
            change: rules::Change::Restrict {
                holder: events::DeclareBlocker::ATTACKER.into(),
                restriction: Self::FLYING_BLOCKERS.restriction(
                    "A creature with flying can’t be blocked except by creatures with flying and/or reach.",
                    (rules::has::<cards::Flying>() | rules::has::<cards::Reach>())
                        .of(events::DeclareBlocker::BLOCKER),
                ),
            },
        })]
    }
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::effects;
    use crate::events;
    use crate::rules;

    use super::Flying;

    #[test]
    fn test_usage() {
        assert_eq!(Flying::keyword().id, Flying::ID);
        assert_eq!(Flying::keyword().abilities(), Flying::abilities());
        assert_eq!(
            Flying::abilities(),
            vec![effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::event::<events::DeclareBlocker>(),
                change: rules::Change::Restrict {
                    holder: events::DeclareBlocker::ATTACKER.into(),
                    restriction: Flying::FLYING_BLOCKERS.restriction(
                        "A creature with flying can’t be blocked except by creatures with flying and/or reach.",
                        (rules::has::<cards::Flying>() | rules::has::<cards::Reach>())
                            .of(events::DeclareBlocker::BLOCKER),
                    ),
                },
            })]
        );
    }
}
