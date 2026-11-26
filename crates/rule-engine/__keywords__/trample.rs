use crate::effects;
use crate::events;
use crate::keywords;
use crate::players;
use crate::rules;

/// 702.19b The controller of an attacking creature with trample first assigns damage to the creature(s) blocking it. Once all those blocking creatures are assigned lethal damage, any excess damage is assigned as its controller chooses among those blocking creatures and the player, planeswalker, or battle the creature is attacking. When checking for assigned lethal damage, take into account damage already marked on the creature and damage from other creatures that’s being assigned during the same combat damage step, but not any abilities or effects that might change the amount of damage that’s actually dealt. The attacking creature’s controller need not assign lethal damage to all those blocking creatures but in that case can’t assign any damage to the player or planeswalker it’s attacking.
pub struct Trample;

impl Trample {
    pub const ID: keywords::KeywordId = keywords::KeywordId("702.19/trample");

    #[must_use]
    pub fn keyword() -> keywords::Keyword {
        keywords::Keyword::from_plain(Self::ID, Self::abilities)
    }

    #[must_use]
    pub fn abilities() -> Vec<effects::Modification> {
        vec![effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::Restriction(events::assign_combat_damage::ONLY_BLOCKERS),
            change: rules::Change::Allow(rules::has::<players::Player>()),
        })]
    }
}

#[cfg(test)]
mod tests {
    use super::Trample;

    #[test]
    fn test_usage() {
        assert_eq!(Trample::keyword().id, Trample::ID);
        assert_eq!(Trample::keyword().abilities(), Trample::abilities());
    }
}
