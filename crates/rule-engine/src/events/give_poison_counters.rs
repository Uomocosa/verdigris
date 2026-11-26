use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::players;
use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct GivePoisonCounters {
    #[arg]
    pub player: players::PlayerId,
    #[arg]
    pub amount: u32,
}

#[rustfmt::skip]
impl rules::traits::context::Context for GivePoisonCounters {
    fn subject(&self) -> Entity { *self.player }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> players::PlayerId { self.player }
}

impl Event for GivePoisonCounters {
    const ID: rules::EventId = rules::EventId("122.1/give-poison-counters");
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::players;
    use crate::rules;
    use crate::rules::traits::context::Context;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    use super::GivePoisonCounters;

    #[test]
    fn test_usage() {
        let player = players::PlayerId(Entity::PLACEHOLDER);
        let event = GivePoisonCounters { player, amount: 3 };
        assert_eq!(event.subject(), *player);
        assert_eq!(event.other(), None);
        assert_eq!(event.player(), player);
        assert_eq!(
            GivePoisonCounters::ID,
            rules::EventId("122.1/give-poison-counters")
        );
        assert_eq!(GivePoisonCounters::restrictions(), Vec::new());
        assert_eq!(
            GivePoisonCounters::args(),
            &[GivePoisonCounters::PLAYER, GivePoisonCounters::AMOUNT,]
        );
    }
}
