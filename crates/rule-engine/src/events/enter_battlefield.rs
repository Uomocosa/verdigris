use atomic_delegate_macros::atomic_delegates;
use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::players;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct EnterBattlefield {
    #[arg]
    pub permanent: Entity,
    #[arg]
    pub controller: players::PlayerId,
    pub summoning_sick: bool,
}

#[atomic_delegates]
impl EnterBattlefield {
    #[must_use]
    pub const fn from_controller(player: players::PlayerId, card: Entity) -> Self {}
}

#[rustfmt::skip]
impl rules::traits::context::Context for EnterBattlefield {
    fn subject(&self) -> Entity { self.permanent }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> players::PlayerId { self.controller }
}

impl Event for EnterBattlefield {
    const ID: rules::EventId = rules::EventId("110.1/enter-battlefield");
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::rules;
    use crate::rules::traits::context::Context;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    use super::EnterBattlefield;
    use crate::players;

    #[test]
    fn test_usage() {
        let permanent = Entity::PLACEHOLDER;
        let controller = players::PlayerId(Entity::from_raw_u32(1).unwrap());
        let event = EnterBattlefield {
            permanent,
            controller,
            summoning_sick: true,
        };
        assert_eq!(event.subject(), permanent);
        assert_eq!(event.other(), None);
        assert_eq!(event.player(), controller);
        assert_eq!(
            EnterBattlefield::ID,
            rules::EventId("110.1/enter-battlefield")
        );
        assert_eq!(EnterBattlefield::restrictions(), Vec::new());
        assert_eq!(event.field(rules::Field::Amount), None);
        assert_eq!(
            EnterBattlefield::args(),
            &[EnterBattlefield::PERMANENT, EnterBattlefield::CONTROLLER,]
        );
    }
}
