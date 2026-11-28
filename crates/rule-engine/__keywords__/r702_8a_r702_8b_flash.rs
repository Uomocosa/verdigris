use crate::effects;
use crate::events;
use crate::keywords;
use crate::rules;

/// 702.8a Flash is a static ability that functions in any zone from which you could play the card
/// it’s on. “Flash” means “You may play this card any time you could cast an instant.”
///
/// 702.8b Multiple instances of flash on the same object are redundant.
#[must_use]
pub fn r702_8a_r702_8b_flash() -> keywords::Keyword {
    keywords::Keyword::from_plain(ID, abilities)
}

pub const ID: keywords::KeywordId = keywords::KeywordId("702.8/flash");

fn abilities() -> Vec<effects::Modification> {
    vec![effects::Modification::Rule(rules::RuleChange {
        target: rules::Target::Restriction(events::cast::TIMING),
        change: rules::Change::AsThough {
            holder: events::Cast::SPELL.into(),
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
        assert_eq!(super::r702_8a_r702_8b_flash().id, super::ID);
        assert_eq!(
            super::r702_8a_r702_8b_flash().abilities(),
            super::abilities()
        );
        assert_eq!(
            super::abilities(),
            vec![effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::Restriction(events::cast::TIMING),
                change: rules::Change::AsThough {
                    holder: events::Cast::SPELL.into(),
                },
            })]
        );
    }
}
