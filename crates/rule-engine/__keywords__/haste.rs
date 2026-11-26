use crate::effects;
use crate::events;
use crate::keywords;
use crate::rules;

/// 702.10a Haste is a static ability.
pub struct Haste;

impl Haste {
    pub const ID: keywords::KeywordId = keywords::KeywordId("702.10/haste");

    #[must_use]
    pub fn keyword() -> keywords::Keyword {
        keywords::Keyword::from_plain(Self::ID, Self::abilities)
    }

    #[must_use]
    pub fn abilities() -> Vec<effects::Modification> {
        vec![effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::Restriction(events::declare_attacker::SUMMONING_SICK),
            change: rules::Change::AsThough {
                holder: events::DeclareAttacker::ATTACKER,
            },
        })]
    }
}

#[cfg(test)]
mod tests {
    use crate::effects;
    use crate::events;
    use crate::rules;

    use super::Haste;

    #[test]
    fn test_usage() {
        assert_eq!(Haste::keyword().id, Haste::ID);
        assert_eq!(Haste::keyword().abilities(), Haste::abilities());
        assert_eq!(
            Haste::abilities(),
            vec![effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::Restriction(events::declare_attacker::SUMMONING_SICK),
                change: rules::Change::AsThough {
                    holder: events::DeclareAttacker::ATTACKER,
                },
            })]
        );
    }
}
