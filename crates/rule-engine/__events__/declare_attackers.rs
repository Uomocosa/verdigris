use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::players;
use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, PartialEq, Eq, Hash, EventArgs)]
pub struct DeclareAttackers {
    #[arg]
    pub attackers: Vec<Entity>,
    #[arg]
    pub player: players::PlayerId,
}

#[rustfmt::skip]
impl Event for DeclareAttackers {
    const ID: rules::EventId = rules::EventId("508.1a/declare-attackers");
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::players;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    use super::DeclareAttackers;

    #[test]
    fn test_usage() {
        let attackers = vec![Entity::PLACEHOLDER];
        let player = players::PlayerId(Entity::PLACEHOLDER);
        let event = DeclareAttackers {
            attackers: attackers.clone(),
            player,
        };
        assert_eq!(
            event.arg(DeclareAttackers::ATTACKERS),
            Some(rules::ArgValue::Entities(attackers))
        );
        assert_eq!(
            event.arg(DeclareAttackers::PLAYER),
            Some(rules::ArgValue::Player(player))
        );
        assert_eq!(
            DeclareAttackers::ID,
            rules::EventId("508.1a/declare-attackers")
        );
        assert_eq!(DeclareAttackers::restrictions(), Vec::new());
        assert_eq!(
            DeclareAttackers::args(),
            &[DeclareAttackers::ATTACKERS, DeclareAttackers::PLAYER,]
        );
    }
}
