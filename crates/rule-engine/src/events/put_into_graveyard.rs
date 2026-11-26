use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::players;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct PutIntoGraveyard {
    #[arg]
    pub object: Entity,
    #[arg]
    pub owner: players::PlayerId,
}

#[rustfmt::skip]
impl rules::traits::context::Context for PutIntoGraveyard {
    fn subject(&self) -> Entity { self.object }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> players::PlayerId { self.owner }
}

impl Event for PutIntoGraveyard {
    const ID: rules::EventId = rules::EventId("404.1/put-into-graveyard");
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::rules;
    use crate::rules::traits::context::Context;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    use super::PutIntoGraveyard;
    use crate::players;

    #[test]
    fn test_usage() {
        let object = Entity::PLACEHOLDER;
        let owner = players::PlayerId(Entity::from_raw_u32(1).unwrap());
        let event = PutIntoGraveyard { object, owner };
        assert_eq!(event.subject(), object);
        assert_eq!(event.other(), None);
        assert_eq!(event.player(), owner);
        assert_eq!(
            PutIntoGraveyard::ID,
            rules::EventId("404.1/put-into-graveyard")
        );
        assert_eq!(PutIntoGraveyard::restrictions(), Vec::new());
        assert_eq!(event.field(rules::Field::Amount), None);
        assert_eq!(
            PutIntoGraveyard::args(),
            &[PutIntoGraveyard::OBJECT, PutIntoGraveyard::OWNER,]
        );
    }
}
