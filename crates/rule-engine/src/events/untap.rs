use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::players;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct Untap {
    #[arg]
    pub permanent: Entity,
    #[arg]
    pub controller: players::PlayerId,
}

#[rustfmt::skip]
impl rules::traits::context::Context for Untap {
    fn subject(&self) -> Entity { self.permanent }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> players::PlayerId { self.controller }
}

impl Event for Untap {
    const ID: rules::EventId = rules::EventId("701.26b/untap");
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::rules;
    use crate::rules::traits::context::Context;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    use super::Untap;
    use crate::players;

    #[test]
    fn test_usage() {
        let permanent = Entity::PLACEHOLDER;
        let controller = players::PlayerId(Entity::from_raw_u32(1).unwrap());
        let event = Untap {
            permanent,
            controller,
        };
        assert_eq!(event.subject(), permanent);
        assert_eq!(event.other(), None);
        assert_eq!(event.player(), controller);
        assert_eq!(Untap::ID, rules::EventId("701.26b/untap"));
        assert_eq!(Untap::restrictions(), Vec::new());
        assert_eq!(event.field(rules::Field::Amount), None);
        assert_eq!(Untap::args(), &[Untap::PERMANENT, Untap::CONTROLLER,]);
    }
}
