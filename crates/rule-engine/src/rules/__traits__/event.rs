use crate::rules;
use crate::rules::traits::legality_rule::LegalityRule;

pub trait Event: Clone + Send + Sync + 'static {
    type Legality: LegalityRule;
    const ID: rules::EventId;
    fn context(self) -> <Self::Legality as LegalityRule>::Ctx;
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::rules;
    use crate::rules::traits::event::Event;

    #[test]
    fn test_usage() {
        let player = Entity::PLACEHOLDER;
        let event = rules::LoseGame(player);
        assert_eq!(event.context(), rules::PlayerCtx(player));
    }
}
