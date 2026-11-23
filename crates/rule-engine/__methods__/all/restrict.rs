use crate::abilities;
use crate::effects;
use crate::rules;
use crate::rules::traits::legality_rule::LegalityRule;
use crate::statics;

pub fn restrict<R: LegalityRule>(
    all: statics::All,
    requirement: rules::Predicate,
) -> abilities::Continuous {
    abilities::Continuous {
        filter: *all,
        change: effects::Change::Restrict(R::ID, requirement),
    }
}

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::effects;
    use crate::rules;
    use crate::statics;
    use crate::testing;

    use super::restrict;

    #[test]
    fn test_usage() {
        let ability =
            restrict::<testing::RestrictedRule>(statics::all(CREATURE), rules::Predicate::True);
        assert_eq!(ability.filter, CREATURE);
        assert_eq!(
            ability.change,
            effects::Change::Restrict(rules::RuleId("test/restriction"), rules::Predicate::True)
        );
    }
}
