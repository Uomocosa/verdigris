use atomic_delegate_macros::atomic_delegate;
use bevy::prelude::Entity;

use crate::players;

use crate::events;
use crate::rules;
use crate::rules::traits::event::Event;

pub const CREATURE: rules::ClauseId<events::DeclareBlocker> =
    rules::ClauseId::new("509.1a/creature");
pub const CONTROLS: rules::ClauseId<events::DeclareBlocker> =
    rules::ClauseId::new("509.1a/controls");
pub const UNTAPPED: rules::ClauseId<events::DeclareBlocker> =
    rules::ClauseId::new("509.1a/untapped");
pub const FLYING: rules::ClauseId<events::DeclareBlocker> = rules::ClauseId::new("702.9b/flying");

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
    fn clauses() -> Vec<rules::Clause> {}
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
    use super::FLYING;
    use super::UNTAPPED;

    #[test]
    fn test_usage() {
        assert_eq!(DeclareBlocker::ID, rules::EventId("509.1a/declare-blocker"));
        let clauses = DeclareBlocker::clauses();
        assert_eq!(clauses.len(), 4);
        assert_eq!(clauses[0].key, CREATURE.key);
        assert_eq!(
            clauses[0].text,
            "The defending player chooses which creatures they control"
        );
        assert_eq!(clauses[0].role, rules::Role::Subject);
        assert_eq!(clauses[0].condition, rules::has::<cards::Creature>());
        assert_eq!(clauses[1].key, CONTROLS.key);
        assert_eq!(clauses[1].text, "creatures they control");
        assert_eq!(clauses[1].role, rules::Role::Subject);
        assert_eq!(
            clauses[1].condition,
            rules::points_to::<cards::Controller>(rules::Role::Player)
                & rules::has::<cards::OnBattlefield>()
        );
        assert_eq!(clauses[2].key, UNTAPPED.key);
        assert_eq!(clauses[2].text, "The chosen creatures must be untapped");
        assert_eq!(clauses[2].role, rules::Role::Subject);
        assert_eq!(clauses[2].condition, !rules::has::<cards::Tapped>());
        assert_eq!(clauses[3].key, FLYING.key);
        assert_eq!(clauses[3].role, rules::Role::Other);
        assert_eq!(
            clauses[3].condition,
            !rules::other(rules::has::<cards::Flying>())
                | rules::has::<cards::Flying>()
                | rules::has::<cards::Reach>()
        );
    }

    #[test]
    fn clause_keys_are_unique_across_declare_rules() {
        let mut seen = HashSet::new();
        for clause in
            DeclareBlocker::clauses().into_iter().chain(events::DeclareAttacker::clauses())
        {
            assert!(
                seen.insert(clause.key),
                "duplicate clause key: {:?}",
                clause.key
            );
        }
    }
}
