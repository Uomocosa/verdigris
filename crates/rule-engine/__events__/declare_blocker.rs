use atomic_delegate_macros::atomic_delegate;
use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::rules;
use crate::rules::traits::event::Event;

pub const CREATURE: rules::RestrictionId = rules::RestrictionId::rule("509.1a/creature");
pub const CONTROLS: rules::RestrictionId = rules::RestrictionId::rule("509.1a/controls");
pub const UNTAPPED: rules::RestrictionId = rules::RestrictionId::rule("509.1a/untapped");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct DeclareBlocker {
    #[arg]
    pub blocker: Entity,
    #[arg]
    pub attacker: Entity,
}

impl Event for DeclareBlocker {
    const ID: rules::EventId = rules::EventId("509.1a/declare-blocker");
    const HOLDER: rules::ArgRef = rules::ArgRef::Arg(Self::BLOCKER);

    #[atomic_delegate(DeclareBlocker, Event)]
    fn restrictions() -> Vec<rules::Restriction> {}
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use bevy::prelude::Entity;

    use crate::cards;
    use crate::events;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    use super::CONTROLS;
    use super::CREATURE;
    use super::DeclareBlocker;
    use super::UNTAPPED;

    #[test]
    fn test_usage() {
        assert_eq!(DeclareBlocker::ID, rules::EventId("509.1a/declare-blocker"));
        assert_eq!(
            DeclareBlocker::HOLDER,
            rules::ArgRef::Arg(DeclareBlocker::BLOCKER)
        );
        let restrictions = DeclareBlocker::restrictions();
        assert_eq!(restrictions.len(), 3);
        assert_eq!(restrictions[0].id, CREATURE);
        assert_eq!(
            restrictions[0].condition,
            rules::has::<cards::Creature>().of(DeclareBlocker::BLOCKER)
        );
        assert_eq!(restrictions[1].id, CONTROLS);
        assert_eq!(
            restrictions[1].condition,
            rules::has::<cards::OnBattlefield>().of(DeclareBlocker::BLOCKER)
        );
        assert_eq!(restrictions[2].id, UNTAPPED);
        assert_eq!(
            restrictions[2].condition,
            (!rules::has::<cards::Tapped>()).of(DeclareBlocker::BLOCKER)
        );
        let event = DeclareBlocker {
            blocker: Entity::PLACEHOLDER,
            attacker: Entity::from_raw_u32(1).unwrap(),
        };
        assert_eq!(
            event.arg(DeclareBlocker::BLOCKER),
            Some(rules::ArgValue::Entity(Entity::PLACEHOLDER))
        );
        assert_eq!(
            DeclareBlocker::args(),
            &[DeclareBlocker::BLOCKER, DeclareBlocker::ATTACKER,]
        );
    }

    #[test]
    fn restriction_ids_are_unique_across_declare_rules() {
        let mut seen = HashSet::new();
        for restriction in DeclareBlocker::restrictions()
            .into_iter()
            .chain(events::DeclareAttackers::restrictions())
            .chain(events::Attack::restrictions())
        {
            assert!(
                seen.insert(restriction.id),
                "duplicate restriction id: {:?}",
                restriction.id
            );
        }
    }
}
