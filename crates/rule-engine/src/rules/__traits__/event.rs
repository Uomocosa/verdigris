use bevy::prelude::{App, Entity};

use crate::Attempt;
use crate::Did;
use crate::May;
use crate::Resolve;
use crate::rules;
use crate::rules::traits::context::Context;

pub trait Event: Context + Clone + Send + Sync + 'static {
    const ID: rules::EventId;
    #[must_use]
    fn clauses() -> Vec<rules::Clause> {
        Vec::new()
    }
    #[must_use]
    fn field(&self, _field: rules::Field) -> Option<u32> {
        None
    }
    #[must_use]
    fn affected(&self) -> Vec<Entity> {
        vec![*self.player()]
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
    use crate::rules::traits::context::Context;
    use crate::rules::traits::event::Event;

    #[test]
    fn test_usage() {
        let player = players::PlayerId(Entity::PLACEHOLDER);
        let event = events::LoseGame(player);
        assert_eq!(event.player(), player);
        assert_eq!(event.affected(), vec![*player]);
        assert_eq!(events::LoseGame::ID, rules::EventId("104.3"));
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
