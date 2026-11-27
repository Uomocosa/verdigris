use crate::rules;

pub trait EventArgs {
    fn arg(&self, id: rules::ArgId) -> Option<rules::ArgValue>;
    fn args() -> &'static [rules::ArgId];
    #[must_use]
    fn kinds() -> &'static [rules::ArgKind] {
        &[]
    }
    #[must_use]
    fn from_args(values: &[rules::ArgValue]) -> Option<Self>
    where
        Self: Sized,
    {
        let _ = values;
        None
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::events;
    use crate::players;
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
        let spell = Entity::from_raw_u32(1).unwrap();
        let caster = players::PlayerId(Entity::from_raw_u32(2).unwrap());
        assert_eq!(
            events::Cast::kinds(),
            &[rules::ArgKind::Entity, rules::ArgKind::Player]
        );
        assert_eq!(
            events::Cast::from_args(&[
                rules::ArgValue::Entity(spell),
                rules::ArgValue::Player(caster),
            ]),
            Some(events::Cast { spell, caster })
        );
        assert_eq!(
            events::Cast::from_args(&[rules::ArgValue::Entity(spell)]),
            None
        );
        assert_eq!(
            events::Cast::from_args(
                &[rules::ArgValue::Amount(1), rules::ArgValue::Player(caster),]
            ),
            None
        );
    }
}
