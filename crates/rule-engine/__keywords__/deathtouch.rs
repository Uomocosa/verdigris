use crate::effects;
use crate::events;
use crate::keywords;
use crate::rules;

/// 702.2c Any nonzero amount of combat damage assigned to a creature by a source with deathtouch is considered to be lethal damage for the purposes of determining if a proposed combat damage assignment is valid, regardless of that creature’s toughness. See rules 510.1c–d.
pub struct Deathtouch;

impl Deathtouch {
    pub const ID: keywords::KeywordId = keywords::KeywordId("702.2/deathtouch");

    #[must_use]
    pub fn keyword() -> keywords::Keyword {
        keywords::Keyword::from_plain(Self::ID, Self::abilities)
    }

    #[must_use]
    pub fn abilities() -> Vec<effects::Modification> {
        vec![effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::Definition(rules::LETHAL_DAMAGE),
            change: rules::Change::Allow {
                holder: events::AssignCombatDamage::SOURCE,
                when: rules::event_field(rules::Field::Amount, rules::Comparison::GreaterThan, 0),
            },
        })]
    }
}

#[cfg(test)]
mod tests {
    use super::Deathtouch;

    #[test]
    fn test_usage() {
        assert_eq!(Deathtouch::keyword().id, Deathtouch::ID);
        assert_eq!(Deathtouch::keyword().abilities(), Deathtouch::abilities());
    }
}
