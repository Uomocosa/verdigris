use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, PartialEq, Eq, Hash, EventArgs)]
pub struct DeclareAttackers {
    #[arg]
    pub attackers: Vec<Entity>,
}

impl Event for DeclareAttackers {
    const ID: rules::EventId = rules::EventId("508.1a/declare-attackers");
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::events;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    use super::DeclareAttackers;

    #[test]
    fn test_usage() {
        assert_eq!(
            DeclareAttackers::ID,
            rules::EventId("508.1a/declare-attackers")
        );
        assert_eq!(
            DeclareAttackers::restrictions(),
            Vec::<rules::Restriction>::new()
        );
        assert_eq!(DeclareAttackers::args(), &[DeclareAttackers::ATTACKERS,]);
        let attackers = vec![Entity::PLACEHOLDER];
        let event = DeclareAttackers {
            attackers: attackers.clone(),
        };
        assert_eq!(
            event.arg(DeclareAttackers::ATTACKERS),
            Some(rules::ArgValue::Entities(attackers))
        );
        assert_eq!(
            events::Attack::restrictions().len(),
            4,
            "per-creature legality lives on Attack",
        );
    }
}
