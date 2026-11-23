use crate::rules;
use crate::rules::traits::context::Context;

pub trait Event: Context + Clone + Send + Sync + 'static {
    const ID: rules::EventId;
    #[must_use]
    fn clauses() -> Vec<rules::Clause> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::rules;
    use crate::rules::traits::context::Context;
    use crate::rules::traits::event::Event;

    #[test]
    fn test_usage() {
        let player = Entity::PLACEHOLDER;
        let event = rules::LoseGame(player);
        assert_eq!(event.player(), player);
        assert_eq!(rules::LoseGame::ID, rules::EventId("104.3"));
    }
}
