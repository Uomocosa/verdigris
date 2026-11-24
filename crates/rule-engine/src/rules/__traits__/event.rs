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
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::players;
    use crate::rules;
    use crate::rules::traits::context::Context;
    use crate::rules::traits::event::Event;

    #[test]
    fn test_usage() {
        let player = players::PlayerId(Entity::PLACEHOLDER);
        let event = rules::LoseGame(player);
        assert_eq!(event.player(), player);
        assert_eq!(rules::LoseGame::ID, rules::EventId("104.3"));
    }
}
