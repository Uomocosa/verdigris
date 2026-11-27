use crate::abilities;
use crate::effects;
use crate::events;
use crate::rules;
use crate::statics;

pub fn attack_without_tapping(all: &statics::All) -> abilities::Continuous {
    let filter: &effects::Filter = all;
    abilities::Continuous {
        filter: filter.clone(),
        as_long_as: None,
        modification: effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::event::<events::TapAttacker>(),
            change: rules::Change::Skip,
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

    use super::attack_without_tapping;

    #[test]
    fn test_usage() {
        let ability = attack_without_tapping(&statics::all(creature()));
        assert_eq!(ability.filter, creature());
        assert_eq!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::event::<events::TapAttacker>(),
                change: rules::Change::Skip,
            }),
        );
    }
}
