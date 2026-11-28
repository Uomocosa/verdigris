use crate::abilities;
use crate::effects;
use crate::rules;
use crate::statics;

pub fn as_though(
    all: &statics::All,
    restriction: rules::RestrictionId,
    holder: impl Into<rules::ArgRef>,
) -> abilities::Continuous {
    abilities::Continuous {
        filter: all.into(),
        as_long_as: None,
        modification: effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::Restriction(restriction),
            change: rules::Change::AsThough {
                holder: holder.into(),
            },
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

    use super::as_though;

    #[test]
    fn test_usage() {
        let ability = as_though(
            &statics::all(creature()),
            testing::UNTAPPED,
            testing::TestRule::ATTACKER,
        );
        assert_eq!(ability.filter, creature());
        assert_eq!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::Restriction(testing::UNTAPPED),
                change: rules::Change::AsThough {
                    holder: testing::TestRule::ATTACKER.into(),
                },
            })
        );
    }
}
