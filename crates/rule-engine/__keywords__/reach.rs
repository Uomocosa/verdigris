use crate::effects;
use crate::keywords;
use crate::rules;

/// 702.17b A creature with flying can’t be blocked except by creatures with flying and/or reach. (See rule 509, “Declare Blockers Step,” and rule 702.9, “Flying.”)
///
/// Reminder text: “This creature can block creatures with flying.”
pub struct Reach;

impl Reach {
    pub const ID: keywords::KeywordId = keywords::KeywordId("702.17/reach");

    #[must_use]
    pub fn keyword() -> keywords::Keyword {
        keywords::Keyword::from_plain(Self::ID, Self::abilities)
    }

    #[must_use]
    pub fn abilities() -> Vec<effects::Modification> {
        vec![effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::Restriction(keywords::Flying::FLYING_BLOCKERS),
            change: rules::Change::AsThough {
                holder: rules::Role::Subject,
            },
        })]
    }
}

#[cfg(test)]
mod tests {
    use crate::effects;
    use crate::keywords;
    use crate::rules;

    use super::Reach;

    #[test]
    fn test_usage() {
        assert_eq!(Reach::keyword().id, Reach::ID);
        assert_eq!(Reach::keyword().abilities(), Reach::abilities());
        assert_eq!(
            Reach::abilities(),
            vec![effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::Restriction(keywords::Flying::FLYING_BLOCKERS),
                change: rules::Change::AsThough {
                    holder: rules::Role::Subject,
                },
            })]
        );
    }
}
