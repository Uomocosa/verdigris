use crate::abilities;
use crate::effects;
use crate::events;
use crate::rules;
use crate::rules::traits::event::Event;
use crate::statics;

pub fn attack_without_tapping(all: &statics::All) -> abilities::Continuous {
    abilities::Continuous {
        filter: all.into(),
        as_long_as: None,
        modification: effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::event::<events::Tap>(),
            change: rules::Change::Skip {
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

    use super::attack_without_tapping;

    #[test]
    fn test_usage() {
        let ability = attack_without_tapping(&statics::all(creature()));
        assert_eq!(ability.filter, creature());
        assert_eq!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::event::<events::Tap>(),
                change: rules::Change::Skip {
                    when: rules::Condition::CausedBy(events::Attack::ID),
                },
            }),
        );
    }
}
