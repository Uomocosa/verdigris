use crate::rules;

pub trait EventArgs {
    fn arg(&self, id: rules::ArgId) -> Option<rules::ArgValue>;
    fn args() -> &'static [rules::ArgId];
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::events;
    use crate::rules;
    use crate::rules::traits::event_args::EventArgs;

    #[test]
    fn test_usage() {
        let blocker = events::DeclareBlocker {
            blocker: Entity::from_raw_u32(1).unwrap(),
            attacker: Entity::PLACEHOLDER,
        };
        assert_eq!(
            blocker.arg(events::DeclareBlocker::BLOCKER),
            Some(rules::ArgValue::Entity(blocker.blocker))
        );
        assert_eq!(
            events::DeclareBlocker::args(),
            &[
                events::DeclareBlocker::BLOCKER,
                events::DeclareBlocker::ATTACKER,
            ]
        );
    }
}
