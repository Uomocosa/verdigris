use verdigris_event_derive::EventArgs;

use crate::players;

use crate::effects;
use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct CreateTokens {
    #[arg]
    pub controller: players::PlayerId,
    #[arg]
    pub amount: u32,
    pub token: effects::TokenDefinition,
}

impl Event for CreateTokens {
    const ID: rules::EventId = rules::EventId("701.7a/create-tokens");
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::effects;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;
    use crate::testing;

    use super::CreateTokens;
    use crate::players;

    #[test]
    fn test_usage() {
        let controller = players::PlayerId(Entity::PLACEHOLDER);
        let event = CreateTokens {
            controller,
            amount: 2,
            token: effects::TokenDefinition::from(testing::SOLDIER_TOKEN.definition),
        };
        assert_eq!(
            event.arg(CreateTokens::CONTROLLER),
            Some(rules::ArgValue::Player(controller))
        );
        assert_eq!(
            event.arg(CreateTokens::CONTROLLER),
            Some(rules::ArgValue::Player(controller))
        );
        assert_eq!(CreateTokens::ID, rules::EventId("701.7a/create-tokens"));
        assert_eq!(CreateTokens::restrictions(), Vec::new());
        assert_eq!(
            CreateTokens::args(),
            &[CreateTokens::CONTROLLER, CreateTokens::AMOUNT,]
        );
    }
}
