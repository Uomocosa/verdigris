use crate::abilities;
use crate::effects;
use crate::rules;
use crate::statics;

pub fn can_block_an_additional_creature(all: &statics::All) -> abilities::Continuous {
    abilities::Continuous {
        filter: all.into(),
        as_long_as: None,
        modification: effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::value::<rules::BlocksPerCreature>(),
            change: rules::Change::Increase(1),
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::creature;
    use crate::effects;
    use crate::rules;
    use crate::statics;

    use super::can_block_an_additional_creature;

    #[test]
    fn test_usage() {
        let ability = can_block_an_additional_creature(&statics::all(creature()));
        assert_eq!(ability.filter, creature());
        assert_eq!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::value::<rules::BlocksPerCreature>(),
                change: rules::Change::Increase(1),
            })
        );
    }
}
