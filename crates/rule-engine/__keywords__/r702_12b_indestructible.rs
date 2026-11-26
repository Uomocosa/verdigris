use crate::effects;
use crate::events;
use crate::keywords;
use crate::rules;

/// 702.12b A permanent with indestructible can’t be destroyed. Such permanents aren’t destroyed by
/// lethal damage, and they ignore the state-based action that checks for lethal damage (see rule
/// 704.5g).
#[must_use]
pub fn r702_12b_indestructible() -> keywords::Keyword {
    keywords::Keyword::from_plain(ID, abilities)
}

pub const ID: keywords::KeywordId = keywords::KeywordId("702.12/indestructible");
pub const CANT_BE_DESTROYED: rules::RestrictionId = rules::RestrictionId("702.12b/indestructible");

fn abilities() -> Vec<effects::Modification> {
    vec![effects::Modification::Rule(rules::RuleChange {
        target: rules::Target::event::<events::Destroy>(),
        change: rules::Change::Restrict {
            holder: events::Destroy::PERMANENT.into(),
            restriction: CANT_BE_DESTROYED.restriction(!rules::Condition::True),
        },
    })]
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_usage() {
        assert_eq!(super::r702_12b_indestructible().id, super::ID);
        assert_eq!(
            super::r702_12b_indestructible().abilities(),
            super::abilities()
        );
    }
}
