use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct AbilityContext {
    #[arg]
    pub ability: Entity,
    #[arg]
    pub source: Entity,
}

#[rustfmt::skip]
impl Event for AbilityContext {
    const ID: rules::EventId = rules::EventId("603.4/ability-context");
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    use super::AbilityContext;

    #[test]
    fn test_usage() {
        let ability = Entity::PLACEHOLDER;
        let source = Entity::from_raw_u32(1).unwrap();
        let event = AbilityContext { ability, source };
        assert_eq!(
            event.arg(AbilityContext::ABILITY),
            Some(rules::ArgValue::Entity(ability))
        );
        assert_eq!(
            event.arg(AbilityContext::SOURCE),
            Some(rules::ArgValue::Entity(source))
        );
        assert_eq!(AbilityContext::ID, rules::EventId("603.4/ability-context"));
        assert_eq!(AbilityContext::restrictions(), Vec::new());
        assert_eq!(
            AbilityContext::args(),
            &[AbilityContext::ABILITY, AbilityContext::SOURCE]
        );
    }
}
