use crate::effects;
use crate::events;
use crate::keywords;
use crate::rules;

/// 702.3a Defender is a static ability.
#[must_use]
pub fn r702_3a_defender() -> keywords::Keyword {
    keywords::Keyword::from_plain(ID, abilities)
}

pub const ID: keywords::KeywordId = keywords::KeywordId("702.3/defender");
pub const CANT_ATTACK: rules::RestrictionId = rules::RestrictionId("702.3b/defender");

fn abilities() -> Vec<effects::Modification> {
    vec![effects::Modification::Rule(rules::RuleChange {
        target: rules::Target::event::<events::DeclareAttacker>(),
        change: rules::Change::Restrict {
            holder: events::DeclareAttacker::ATTACKER.into(),
            restriction: CANT_ATTACK.restriction(!rules::Condition::True),
        },
    })]
}

#[cfg(test)]
mod tests {
    use crate::effects;
    use crate::events;
    use crate::rules;

    #[test]
    fn test_usage() {
        assert_eq!(super::r702_3a_defender().id, super::ID);
        assert_eq!(super::r702_3a_defender().abilities(), super::abilities());
        assert_eq!(
            super::abilities(),
            vec![effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::event::<events::DeclareAttacker>(),
                change: rules::Change::Restrict {
                    holder: events::DeclareAttacker::ATTACKER.into(),
                    restriction: super::CANT_ATTACK.restriction(!rules::Condition::True,),
                },
            })]
        );
    }
}
