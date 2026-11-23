use bevy::prelude::Entity;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EnterBattlefield {
    pub permanent: Entity,
    pub controller: Entity,
    pub summoning_sick: bool,
}

#[rustfmt::skip]
impl rules::traits::context::Context for EnterBattlefield {
    fn subject(&self) -> Entity { self.permanent }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> Entity { self.controller }
}

impl Event for EnterBattlefield {
    const ID: rules::EventId = rules::EventId("110.1");
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::rules;
    use crate::rules::traits::context::Context;
    use crate::rules::traits::event::Event;

    use super::EnterBattlefield;

    #[test]
    fn test_usage() {
        let permanent = Entity::PLACEHOLDER;
        let controller = Entity::from_raw_u32(1).unwrap();
        let event = EnterBattlefield {
            permanent,
            controller,
            summoning_sick: true,
        };
        assert_eq!(event.subject(), permanent);
        assert_eq!(event.other(), None);
        assert_eq!(event.player(), controller);
        assert_eq!(EnterBattlefield::ID, rules::EventId("110.1"));
        assert_eq!(EnterBattlefield::clauses(), Vec::new());
        assert_eq!(event.field(rules::Field::Amount), None);
    }
}
