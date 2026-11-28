use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct Create {
    #[arg]
    pub token: Entity,
}

impl Event for Create {
    const ID: rules::EventId = rules::EventId("111.1/create");
    const HOLDER: rules::ArgRef = rules::ArgRef::Arg(Self::TOKEN);
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    use super::Create;

    #[test]
    fn test_usage() {
        let token = Entity::PLACEHOLDER;
        let event = Create { token };
        assert_eq!(
            event.arg(Create::TOKEN),
            Some(rules::ArgValue::Entity(token))
        );
        assert_eq!(Create::ID, rules::EventId("111.1/create"));
        assert_eq!(Create::HOLDER, rules::ArgRef::Arg(Create::TOKEN));
        assert_eq!(Create::restrictions(), Vec::new());
        assert_eq!(Create::args(), &[Create::TOKEN,]);
    }
}
