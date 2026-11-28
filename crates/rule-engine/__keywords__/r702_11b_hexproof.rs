use crate::cards;
use crate::effects;
use crate::events;
use crate::keywords;
use crate::rules;

/// 702.11b “Hexproof” on a permanent means “This permanent can’t be the target of spells or
/// abilities your opponents control.”
#[must_use]
pub fn r702_11b_hexproof() -> keywords::Keyword {
    keywords::Keyword::from_plain(ID, abilities)
}

pub const ID: keywords::KeywordId = keywords::KeywordId("702.11/hexproof");
pub const CANT_BE_TARGETED: rules::RestrictionId = rules::RestrictionId::rule("702.11b/hexproof");

fn abilities() -> Vec<effects::Modification> {
    vec![effects::Modification::Rule(rules::RuleChange {
        target: rules::Target::event::<events::BecomeTarget>(),
        change: rules::Change::Restrict {
            holder: events::BecomeTarget::TARGET.into(),
            restriction: CANT_BE_TARGETED.restriction(
                (rules::points_to::<cards::Controller>(
                    events::BecomeTarget::SPELL_OR_ABILITY.controller(),
                ) | rules::is(events::BecomeTarget::SPELL_OR_ABILITY.controller()))
                .of(events::BecomeTarget::TARGET),
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
        assert_eq!(super::r702_11b_hexproof().id, super::ID);
        assert_eq!(super::r702_11b_hexproof().abilities(), super::abilities());
        assert_eq!(
            super::abilities(),
            vec![effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::event::<events::BecomeTarget>(),
                change: rules::Change::Restrict {
                    holder: events::BecomeTarget::TARGET.into(),
                    restriction: super::CANT_BE_TARGETED.restriction(
                        (rules::points_to::<cards::Controller>(
                            events::BecomeTarget::SPELL_OR_ABILITY.controller()
                        ) | rules::is(events::BecomeTarget::SPELL_OR_ABILITY.controller()))
                        .of(events::BecomeTarget::TARGET),
                    ),
                },
            })]
        );
    }
}
