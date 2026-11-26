use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::players;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct Exile {
    #[arg]
    pub object: Entity,
    #[arg]
    pub owner: players::PlayerId,
}

#[rustfmt::skip]
impl rules::traits::context::Context for Exile {
    fn subject(&self) -> Entity { self.object }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> players::PlayerId { self.owner }
}

impl Event for Exile {
    const ID: rules::EventId = rules::EventId("701.13a/exile");
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::rules;
    use crate::rules::traits::context::Context;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    use super::Exile;
    use crate::players;

    #[test]
    fn test_usage() {
        let object = Entity::PLACEHOLDER;
        let owner = players::PlayerId(Entity::from_raw_u32(1).unwrap());
        let event = Exile { object, owner };
        assert_eq!(event.subject(), object);
        assert_eq!(event.other(), None);
        assert_eq!(event.player(), owner);
        assert_eq!(Exile::ID, rules::EventId("701.13a/exile"));
        assert_eq!(Exile::restrictions(), Vec::new());
        assert_eq!(Exile::args(), &[Exile::OBJECT, Exile::OWNER,]);
    }
}
