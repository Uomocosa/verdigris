use atomic_delegate_macros::atomic_delegate;
use bevy::prelude::Entity;

use crate::players;

use crate::rules;
use crate::rules::traits::event::Event;

pub const CREATURE: rules::RestrictionId = rules::RestrictionId("508.1a/creature");
pub const CONTROLS: rules::RestrictionId = rules::RestrictionId("508.1a/controls");
pub const UNTAPPED: rules::RestrictionId = rules::RestrictionId("508.1a/untapped");
pub const SUMMONING_SICK: rules::RestrictionId =
    rules::RestrictionId("508.1a/controlled-since-turn-began");
pub const DEFENDER: rules::RestrictionId = rules::RestrictionId("702.3b/defender");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeclareAttacker {
    pub attacker: Entity,
    pub player: players::PlayerId,
}

#[rustfmt::skip]
impl rules::traits::context::Context for DeclareAttacker {
    fn subject(&self) -> Entity { self.attacker }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> players::PlayerId { self.player }
}

impl Event for DeclareAttacker {
    const ID: rules::EventId = rules::EventId("508.1a/declare-attacker");

    #[atomic_delegate(DeclareAttacker, Event)]
    fn restrictions() -> Vec<rules::Restriction> {}
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::events;
    use crate::rules;
    use crate::rules::traits::event::Event;

    use super::CONTROLS;
    use super::CREATURE;
    use super::DEFENDER;
    use super::DeclareAttacker;
    use super::SUMMONING_SICK;
    use super::UNTAPPED;

    #[test]
    fn test_usage() {
        assert_eq!(
            DeclareAttacker::ID,
            rules::EventId("508.1a/declare-attacker")
        );
        let restrictions = DeclareAttacker::restrictions();
        assert_eq!(restrictions.len(), 5);
        assert_eq!(restrictions[0].id, CREATURE);
        assert_eq!(
            restrictions[0].text,
            "The active player chooses which creatures that they control"
        );
        assert_eq!(restrictions[0].role, rules::Role::Subject);
        assert_eq!(restrictions[0].condition, rules::has::<cards::Creature>());
        assert_eq!(restrictions[1].id, CONTROLS);
        assert_eq!(restrictions[1].text, "creatures that they control");
        assert_eq!(restrictions[1].role, rules::Role::Subject);
        assert_eq!(
            restrictions[1].condition,
            rules::points_to::<cards::Controller>(rules::Role::Player)
                & rules::has::<cards::OnBattlefield>()
        );
        assert_eq!(restrictions[2].id, UNTAPPED);
        assert_eq!(
            restrictions[2].text,
            "The chosen creatures must be untapped"
        );
        assert_eq!(restrictions[2].role, rules::Role::Subject);
        assert_eq!(restrictions[2].condition, !rules::has::<cards::Tapped>());
        assert_eq!(restrictions[3].id, SUMMONING_SICK);
        assert_eq!(restrictions[3].role, rules::Role::Subject);
        assert_eq!(
            restrictions[3].condition,
            !rules::has::<cards::SummoningSick>()
        );
        assert_eq!(restrictions[4].id, DEFENDER);
        assert_eq!(
            restrictions[4].text,
            "A creature with defender can’t attack."
        );
        assert_eq!(restrictions[4].role, rules::Role::Subject);
        assert_eq!(restrictions[4].condition, !rules::has::<cards::Defender>());
    }

    #[test]
    fn restriction_texts_occur_verbatim_in_comprehensive_rules() {
        const RULES_TEXT: &str = include_str!("../../../../docs/MagicCompRules-20250919.txt");
        for restriction in events::DeclareBlocker::restrictions()
            .into_iter()
            .chain(DeclareAttacker::restrictions())
        {
            assert!(
                RULES_TEXT.contains(restriction.text),
                "restriction {:?} text not found verbatim in Comprehensive Rules: {:?}",
                restriction.id,
                restriction.text
            );
        }
    }
}
