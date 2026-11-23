use crate::abilities;
use crate::effects;
use crate::rules;
use crate::statics;

pub fn allow<Ctx>(
    all: statics::All,
    clause: rules::ClauseId<Ctx>,
    alternative: rules::Predicate,
) -> abilities::Continuous {
    abilities::Continuous {
        filter: *all,
        modification: effects::Modification::Rule(rules::RuleChange::Allow(
            clause.key,
            alternative,
        )),
    }
}

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::cards;
    use crate::effects;
    use crate::rules;
    use crate::statics;

    use super::allow;

    #[test]
    fn test_usage() {
        let ability = allow(
            statics::all(CREATURE),
            rules::ClauseId::<rules::AttackCtx>::new("test/untapped"),
            rules::has::<cards::Flying>(),
        );
        assert_eq!(ability.filter, CREATURE);
        assert_eq!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange::Allow(
                rules::ClauseKey("test/untapped"),
                rules::has::<cards::Flying>(),
            ))
        );
    }
}
