use crate::abilities;
use crate::effects;
use crate::rules::traits::legality_rule::LegalityRule;
use crate::statics;

pub fn requirement<R: LegalityRule>(all: statics::All) -> abilities::Continuous {
    abilities::Continuous {
        filter: *all,
        modification: effects::Modification::Requirement(R::ID),
    }
}

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::effects;
    use crate::rules;
    use crate::statics;
    use crate::testing;

    use super::requirement;

    #[test]
    fn test_usage() {
        let ability = requirement::<testing::RestrictedRule>(statics::all(CREATURE));
        assert_eq!(ability.filter, CREATURE);
        assert_eq!(
            ability.modification,
            effects::Modification::Requirement(rules::RuleId("test/restriction"))
        );
    }
}
