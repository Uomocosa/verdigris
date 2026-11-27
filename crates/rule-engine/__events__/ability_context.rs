use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct AbilityContext {
    #[arg]
    pub spell_or_ability: Entity,
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
        let event = AbilityContext {
            spell_or_ability: ability,
        };
        assert_eq!(
            event.arg(AbilityContext::SPELL_OR_ABILITY),
            Some(rules::ArgValue::Entity(ability))
        );
        assert_eq!(AbilityContext::ID, rules::EventId("603.4/ability-context"));
        assert_eq!(AbilityContext::restrictions(), Vec::new());
        assert_eq!(AbilityContext::args(), &[AbilityContext::SPELL_OR_ABILITY]);
    }
}
