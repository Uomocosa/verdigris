use crate::abilities;
use crate::effects;
use crate::rules;
use crate::rules::traits::event::Event;
use crate::statics;

pub fn requirement<E: Event>(all: &statics::All) -> abilities::Continuous {
    let filter: &effects::Filter = all;
    abilities::Continuous {
        filter: filter.clone(),
        as_long_as: None,
        modification: effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::event::<E>(),
            change: rules::Change::Must,
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::creature;
    use crate::effects;
    use crate::rules;
    use crate::statics;
    use crate::testing;

    use super::requirement;

    #[test]
    fn test_usage() {
        let ability = requirement::<testing::RestrictedRule>(&statics::all(creature()));
        assert_eq!(ability.filter, creature());
        assert_eq!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::event::<testing::RestrictedRule>(),
                change: rules::Change::Must,
            })
        );
    }
}
