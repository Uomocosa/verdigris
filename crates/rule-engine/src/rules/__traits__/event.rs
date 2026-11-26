use bevy::prelude::{App, Entity};

use crate::Attempt;
use crate::Did;
use crate::May;
use crate::Resolve;
use crate::rules;
use crate::rules::traits::event_args::EventArgs;

pub trait Event: EventArgs + Clone + Send + Sync + 'static {
    const ID: rules::EventId;
    #[must_use]
    fn restrictions() -> Vec<rules::Restriction> {
        Vec::new()
    }
    #[must_use]
    fn affected(&self) -> Vec<Entity> {
        Self::args()
            .iter()
            .filter_map(|id| match self.arg(*id) {
                Some(rules::ArgValue::Player(player)) => Some(*player),
                _ => None,
            })
            .collect()
    }
    fn register(app: &mut App) {
        app.add_message::<Attempt<Self>>()
            .add_message::<Resolve<Self>>()
            .add_message::<May<Self>>()
            .add_message::<Did<Self>>();
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::events;
    use crate::players;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    #[test]
    fn test_usage() {
        let player = players::PlayerId(Entity::PLACEHOLDER);
        let event = events::LoseGame { player };
        assert_eq!(
            event.arg(events::LoseGame::PLAYER),
            Some(rules::ArgValue::Player(player))
        );
        assert_eq!(event.affected(), vec![*player]);
        assert_eq!(events::LoseGame::ID, rules::EventId("104.3/lose-game"));
    }

    #[test]
    fn deal_damage_affects_its_recipient_only() {
        let source = Entity::from_raw_u32(1).unwrap();
        let recipient = Entity::from_raw_u32(2).unwrap();
        let controller = players::PlayerId(Entity::from_raw_u32(3).unwrap());
        let event = events::DealDamage {
            source,
            recipient,
            amount: 1,
            controller,
        };
        assert_eq!(event.affected(), vec![recipient]);
    }
}
