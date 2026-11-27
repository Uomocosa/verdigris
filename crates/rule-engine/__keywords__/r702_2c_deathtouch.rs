use crate::effects;
use crate::events;
use crate::keywords;
use crate::rules;

/// 702.2c Any nonzero amount of combat damage assigned to a creature by a source with deathtouch is
/// considered to be lethal damage for the purposes of determining if a proposed combat damage
/// assignment is valid, regardless of that creature’s toughness. See rules 510.1c–d.
#[must_use]
pub fn r702_2c_deathtouch() -> keywords::Keyword {
    keywords::Keyword::from_plain(ID, abilities)
}

pub const ID: keywords::KeywordId = keywords::KeywordId("702.2/deathtouch");

fn abilities() -> Vec<effects::Modification> {
    vec![effects::Modification::Rule(rules::RuleChange {
        target: rules::Target::Definition(rules::LETHAL_DAMAGE),
        change: rules::Change::Allow {
            holder: events::AssignCombatDamage::SOURCE.into(),
            when: events::AssignCombatDamage::AMOUNT.is(rules::greater_than(0)),
        },
    })]
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_usage() {
        assert_eq!(super::r702_2c_deathtouch().id, super::ID);
        assert_eq!(super::r702_2c_deathtouch().abilities(), super::abilities());
    }
}
