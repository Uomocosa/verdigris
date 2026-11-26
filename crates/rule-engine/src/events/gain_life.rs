use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::players;
use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct GainLife {
    #[arg]
    pub player: players::PlayerId,
    #[arg]
    pub amount: u32,
}

#[rustfmt::skip]
impl rules::traits::context::Context for GainLife {
    fn subject(&self) -> Entity { *self.player }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> players::PlayerId { self.player }
}

impl Event for GainLife {
    const ID: rules::EventId = rules::EventId("119.3/gain-life");
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::players;
    use crate::rules;
    use crate::rules::traits::context::Context;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    use super::GainLife;

    #[test]
    fn test_usage() {
        let player = players::PlayerId(Entity::PLACEHOLDER);
        let event = GainLife { player, amount: 2 };
        assert_eq!(event.subject(), *player);
        assert_eq!(event.other(), None);
        assert_eq!(event.player(), player);
        assert_eq!(GainLife::ID, rules::EventId("119.3/gain-life"));
        assert_eq!(GainLife::restrictions(), Vec::new());
        assert_eq!(GainLife::args(), &[GainLife::PLAYER, GainLife::AMOUNT,]);
    }
}
