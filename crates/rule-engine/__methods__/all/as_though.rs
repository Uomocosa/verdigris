use crate::abilities;
use crate::effects;
use crate::rules;
use crate::statics;

pub fn as_though<Ctx>(all: statics::All, clause: rules::ClauseId<Ctx>) -> abilities::Continuous {
    abilities::Continuous {
        filter: *all,
        modification: effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::Clause(clause.key),
            change: rules::Change::AsThough,
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
            rules::ClauseId::<rules::DeclareAttacker>::new("test/untapped"),
        );
        assert_eq!(ability.filter, CREATURE);
        assert_eq!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::Clause(rules::ClauseKey("test/untapped")),
                change: rules::Change::AsThough,
            })
        );
    }
}
