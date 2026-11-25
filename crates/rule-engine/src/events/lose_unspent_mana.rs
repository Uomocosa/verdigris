use bevy::prelude::Entity;
use derive_more::Deref;

use crate::players;
use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref)]
pub struct LoseUnspentMana(pub players::PlayerId);

#[rustfmt::skip]
impl rules::traits::context::Context for LoseUnspentMana {
    fn subject(&self) -> Entity { ***self }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> players::PlayerId { **self }
}

#[rustfmt::skip]
impl Event for LoseUnspentMana {
    const ID: rules::EventId = rules::EventId("106.4");
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::players;
    use crate::rules;
    use crate::rules::traits::context::Context;
    use crate::rules::traits::event::Event;

    use super::LoseUnspentMana;

    #[test]
    fn test_usage() {
        let player = players::PlayerId(Entity::PLACEHOLDER);
        let event = LoseUnspentMana(player);
        assert_eq!(event.subject(), *player);
        assert_eq!(event.other(), None);
        assert_eq!(event.player(), player);
        assert_eq!(LoseUnspentMana::ID, rules::EventId("106.4"));
        assert_eq!(LoseUnspentMana::clauses(), Vec::new());
    }
}
