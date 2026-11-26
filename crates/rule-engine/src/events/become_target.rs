use bevy::prelude::Entity;

use crate::players;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BecomeTarget {
    pub target: Entity,
    pub spell: Entity,
    pub player: players::PlayerId,
}

#[rustfmt::skip]
impl rules::traits::context::Context for BecomeTarget {
    fn subject(&self) -> Entity { self.target }
    fn other(&self) -> Option<Entity> { Some(self.spell) }
    fn player(&self) -> players::PlayerId { self.player }
}

impl Event for BecomeTarget {
    const ID: rules::EventId = rules::EventId("601.2c/become-target");
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::players;
    use crate::rules;
    use crate::rules::traits::context::Context;
    use crate::rules::traits::event::Event;

    use super::BecomeTarget;

    #[test]
    fn test_usage() {
        let target = Entity::PLACEHOLDER;
        let spell = Entity::from_raw_u32(1).unwrap();
        let player = players::PlayerId(Entity::from_raw_u32(2).unwrap());
        let event = BecomeTarget {
            target,
            spell,
            player,
        };
        assert_eq!(event.subject(), target);
        assert_eq!(event.other(), Some(spell));
        assert_eq!(event.player(), player);
        assert_eq!(BecomeTarget::ID, rules::EventId("601.2c/become-target"));
        assert_eq!(BecomeTarget::restrictions(), Vec::new());
    }
}
