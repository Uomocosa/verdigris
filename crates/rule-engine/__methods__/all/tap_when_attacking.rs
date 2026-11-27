use crate::abilities;
use crate::effects;
use crate::events;
use crate::rules;
use crate::statics;

pub fn tap_when_attacking(all: &statics::All) -> abilities::Continuous {
    let filter: &effects::Filter = all;
    abilities::Continuous {
        filter: filter.clone(),
        as_long_as: None,
        modification: effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::event::<events::TapAttacker>(),
            change: rules::Change::Perform,
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::creature;
    use crate::effects;
    use crate::events;
    use crate::rules;
    use crate::statics;

    use super::tap_when_attacking;

    #[test]
    fn test_usage() {
        let ability = tap_when_attacking(&statics::all(creature()));
        assert_eq!(ability.filter, creature());
        assert_eq!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::event::<events::TapAttacker>(),
                change: rules::Change::Perform,
            }),
        );
    }
}
