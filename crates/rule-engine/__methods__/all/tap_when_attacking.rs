use crate::abilities;
use crate::effects;
use crate::events;
use crate::rules;
use crate::rules::traits::event::Event;
use crate::statics;

pub fn tap_when_attacking(all: &statics::All) -> abilities::Continuous {
    abilities::Continuous {
        filter: all.into(),
        as_long_as: None,
        modification: effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::event::<events::Tap>(),
            change: rules::Change::Perform {
                when: rules::Condition::CausedBy(events::Attack::ID),
            },
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::creature;
    use crate::effects;
    use crate::events;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::statics;

    use super::tap_when_attacking;

    #[test]
    fn test_usage() {
        let ability = tap_when_attacking(&statics::all(creature()));
        assert_eq!(ability.filter, creature());
        assert_eq!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::event::<events::Tap>(),
                change: rules::Change::Perform {
                    when: rules::Condition::CausedBy(events::Attack::ID),
                },
            }),
        );
    }
}
