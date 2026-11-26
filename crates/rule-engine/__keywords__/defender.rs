use crate::effects;
use crate::events;
use crate::keywords;
use crate::rules;

/// 702.3a Defender is a static ability.
pub struct Defender;

impl Defender {
    pub const ID: keywords::KeywordId = keywords::KeywordId("702.3/defender");
    pub const CANT_ATTACK: rules::RestrictionId = rules::RestrictionId("702.3b/defender");

    #[must_use]
    pub fn keyword() -> keywords::Keyword {
        keywords::Keyword::from_plain(Self::ID, Self::abilities)
    }

    #[must_use]
    pub fn abilities() -> Vec<effects::Modification> {
        vec![effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::event::<events::DeclareAttacker>(),
            change: rules::Change::Restrict {
                holder: events::DeclareAttacker::ATTACKER.into(),
                restriction: Self::CANT_ATTACK.restriction(!rules::Condition::True),
            },
        })]
    }
}

#[cfg(test)]
mod tests {
    use crate::effects;
    use crate::events;
    use crate::rules;

    use super::Defender;

    #[test]
    fn test_usage() {
        assert_eq!(Defender::keyword().id, Defender::ID);
        assert_eq!(Defender::keyword().abilities(), Defender::abilities());
        assert_eq!(
            Defender::abilities(),
            vec![effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::event::<events::DeclareAttacker>(),
                change: rules::Change::Restrict {
                    holder: events::DeclareAttacker::ATTACKER.into(),
                    restriction: Defender::CANT_ATTACK.restriction(!rules::Condition::True,),
                },
            })]
        );
    }
}
