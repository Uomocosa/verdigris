use crate::abilities;
use crate::effects;
use crate::rules;
use crate::statics;

pub fn as_though(
    all: statics::All,
    restriction: rules::RestrictionId,
    holder: rules::Role,
) -> abilities::Continuous {
    abilities::Continuous {
        filter: *all,
        modification: effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::Restriction(restriction),
            change: rules::Change::AsThough { holder },
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::effects;
    use crate::rules;
    use crate::statics;

    use super::as_though;

    #[test]
    fn test_usage() {
        let ability = as_though(
            statics::all(CREATURE),
            rules::RestrictionId("test/untapped"),
            rules::Role::Subject,
        );
        assert_eq!(ability.filter, CREATURE);
        assert_eq!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::Restriction(rules::RestrictionId("test/untapped")),
                change: rules::Change::AsThough {
                    holder: rules::Role::Subject,
                },
            })
        );
    }
}
