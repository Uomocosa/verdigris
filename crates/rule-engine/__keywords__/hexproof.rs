use crate::cards;
use crate::effects;
use crate::events;
use crate::keywords;
use crate::rules;

/// 702.11b “Hexproof” on a permanent means “This permanent can’t be the target of spells or abilities your opponents control.”
pub struct Hexproof;

impl Hexproof {
    pub const ID: keywords::KeywordId = keywords::KeywordId("702.11/hexproof");
    pub const CANT_BE_TARGETED: rules::RestrictionId = rules::RestrictionId("702.11b/hexproof");

    #[must_use]
    pub fn keyword() -> keywords::Keyword {
        keywords::Keyword::from_plain(Self::ID, Self::abilities)
    }

    #[must_use]
    pub fn abilities() -> Vec<effects::Modification> {
        vec![effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::event::<events::BecomeTarget>(),
            change: rules::Change::Restrict {
                holder: events::BecomeTarget::TARGET,
                restriction: Self::CANT_BE_TARGETED.restriction(
                    "can’t be the target of spells or abilities your opponents control",
                    rules::points_to::<cards::Controller>(rules::Role::Player)
                        | rules::is(rules::Role::Player),
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

    use super::Hexproof;

    #[test]
    fn test_usage() {
        assert_eq!(Hexproof::keyword().id, Hexproof::ID);
        assert_eq!(Hexproof::keyword().abilities(), Hexproof::abilities());
        assert_eq!(
            Hexproof::abilities(),
            vec![effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::event::<events::BecomeTarget>(),
                change: rules::Change::Restrict {
                    holder: events::BecomeTarget::TARGET,
                    restriction: Hexproof::CANT_BE_TARGETED.restriction(
                        "can’t be the target of spells or abilities your opponents control",
                        rules::points_to::<cards::Controller>(rules::Role::Player)
                            | rules::is(rules::Role::Player),
                    ),
                },
            })]
        );
    }
}
