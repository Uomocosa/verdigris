use crate::effects;
use crate::events;
use crate::keywords;
use crate::rules;

/// 702.12b A permanent with indestructible can’t be destroyed. Such permanents aren’t destroyed by lethal damage, and they ignore the state-based action that checks for lethal damage (see rule 704.5g).
pub struct Indestructible;

impl Indestructible {
    pub const ID: keywords::KeywordId = keywords::KeywordId("702.12/indestructible");
    pub const CANT_BE_DESTROYED: rules::RestrictionId =
        rules::RestrictionId("702.12b/indestructible");

    #[must_use]
    pub fn keyword() -> keywords::Keyword {
        keywords::Keyword::from_plain(Self::ID, Self::abilities)
    }

    #[must_use]
    pub fn abilities() -> Vec<effects::Modification> {
        vec![effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::event::<events::Destroy>(),
            change: rules::Change::Restrict {
                holder: events::Destroy::PERMANENT.into(),
                restriction: Self::CANT_BE_DESTROYED
                    .restriction("can’t be destroyed", !rules::Condition::True),
            },
        })]
    }
}

#[cfg(test)]
mod tests {
    use super::Indestructible;

    #[test]
    fn test_usage() {
        assert_eq!(Indestructible::keyword().id, Indestructible::ID);
        assert_eq!(
            Indestructible::keyword().abilities(),
            Indestructible::abilities()
        );
    }
}
