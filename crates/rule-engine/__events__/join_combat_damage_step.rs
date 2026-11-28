use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct JoinCombatDamageStep {
    #[arg]
    pub creature: Entity,
}

impl Event for JoinCombatDamageStep {
    const ID: rules::EventId = rules::EventId("510.4/join-combat-damage-step");
    const HOLDER: rules::ArgRef = rules::ArgRef::Arg(Self::CREATURE);
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    use super::JoinCombatDamageStep;

    #[test]
    fn test_usage() {
        let creature = Entity::PLACEHOLDER;
        let event = JoinCombatDamageStep { creature };
        assert_eq!(
            event.arg(JoinCombatDamageStep::CREATURE),
            Some(rules::ArgValue::Entity(creature))
        );
        assert_eq!(
            JoinCombatDamageStep::ID,
            rules::EventId("510.4/join-combat-damage-step")
        );
        assert_eq!(
            JoinCombatDamageStep::HOLDER,
            rules::ArgRef::Arg(JoinCombatDamageStep::CREATURE)
        );
        assert_eq!(JoinCombatDamageStep::restrictions(), Vec::new());
        assert_eq!(
            JoinCombatDamageStep::args(),
            &[JoinCombatDamageStep::CREATURE,]
        );
    }
}
