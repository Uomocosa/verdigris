use bevy::prelude::App;

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
        assert_eq!(events::LoseGame::ID, rules::EventId("104.3"));
    }
}
