use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct EnterTheBattlefield {
    #[arg]
    pub permanent: Entity,
}

impl Event for EnterTheBattlefield {
    const ID: rules::EventId = rules::EventId("110.1/enter-the-battlefield");
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    use super::EnterTheBattlefield;

    #[test]
    fn test_usage() {
        let permanent = Entity::PLACEHOLDER;
        let event = EnterTheBattlefield { permanent };
        assert_eq!(
            event.arg(EnterTheBattlefield::PERMANENT),
            Some(rules::ArgValue::Entity(permanent))
        );
        assert_eq!(
            EnterTheBattlefield::ID,
            rules::EventId("110.1/enter-the-battlefield")
        );
        assert_eq!(EnterTheBattlefield::restrictions(), Vec::new());
        assert_eq!(
            EnterTheBattlefield::args(),
            &[EnterTheBattlefield::PERMANENT,]
        );
    }
}
