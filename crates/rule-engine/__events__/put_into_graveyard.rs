use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct PutIntoGraveyard {
    #[arg]
    pub object: Entity,
}

impl Event for PutIntoGraveyard {
    const ID: rules::EventId = rules::EventId("404.1/put-into-graveyard");
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    use super::PutIntoGraveyard;

    #[test]
    fn test_usage() {
        let object = Entity::PLACEHOLDER;
        let event = PutIntoGraveyard { object };
        assert_eq!(
            event.arg(PutIntoGraveyard::OBJECT),
            Some(rules::ArgValue::Entity(object))
        );
        assert_eq!(
            PutIntoGraveyard::ID,
            rules::EventId("404.1/put-into-graveyard")
        );
        assert_eq!(PutIntoGraveyard::restrictions(), Vec::new());
        assert_eq!(PutIntoGraveyard::args(), &[PutIntoGraveyard::OBJECT,]);
    }
}
