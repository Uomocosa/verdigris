use atomic_delegate_macros::atomic_delegate;
use bevy::prelude::Entity;

use crate::players;

use crate::rules;
use crate::rules::traits::event::Event;

pub const CREATURE: rules::RestrictionId = rules::RestrictionId("509.1a/creature");
pub const CONTROLS: rules::RestrictionId = rules::RestrictionId("509.1a/controls");
pub const UNTAPPED: rules::RestrictionId = rules::RestrictionId("509.1a/untapped");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeclareBlocker {
    pub blocker: Entity,
    pub attacker: Entity,
    pub player: players::PlayerId,
}

#[rustfmt::skip]
impl rules::traits::context::Context for DeclareBlocker {
    fn subject(&self) -> Entity { self.blocker }
    fn other(&self) -> Option<Entity> { Some(self.attacker) }
    fn player(&self) -> players::PlayerId { self.player }
}

impl Event for DeclareBlocker {
    const ID: rules::EventId = rules::EventId("509.1a/declare-blocker");

    #[atomic_delegate(DeclareBlocker, Event)]
    fn restrictions() -> Vec<rules::Restriction> {}
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::cards;
    use crate::events;
    use crate::rules;
    use crate::rules::traits::event::Event;

    use super::CONTROLS;
    use super::CREATURE;
    use super::DeclareBlocker;
    use super::UNTAPPED;

    #[test]
    fn test_usage() {
        assert_eq!(DeclareBlocker::ID, rules::EventId("509.1a/declare-blocker"));
        let restrictions = DeclareBlocker::restrictions();
        assert_eq!(restrictions.len(), 3);
        assert_eq!(restrictions[0].id, CREATURE);
        assert_eq!(
            restrictions[0].text,
            "The defending player chooses which creatures they control"
        );
        assert_eq!(restrictions[0].condition, rules::has::<cards::Creature>());
        assert_eq!(restrictions[1].id, CONTROLS);
        assert_eq!(restrictions[1].text, "creatures they control");
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
        assert_eq!(restrictions[2].condition, !rules::has::<cards::Tapped>());
    }

    #[test]
    fn restriction_ids_are_unique_across_declare_rules() {
        let mut seen = HashSet::new();
        for restriction in DeclareBlocker::restrictions()
            .into_iter()
            .chain(events::DeclareAttacker::restrictions())
        {
            assert!(
                seen.insert(restriction.id),
                "duplicate restriction id: {:?}",
                restriction.id
            );
        }
    }
}
