use crate::abilities;
use crate::effects;
use crate::rules::traits::legality_rule::LegalityRule;
use crate::statics;

pub fn demand<R: LegalityRule>(all: statics::All) -> abilities::Continuous {
    abilities::Continuous {
        filter: *all,
        change: effects::Change::Demand(R::ID),
    }
}

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::effects;
    use crate::rules;
    use crate::statics;
    use crate::testing;

    use super::demand;

    #[test]
    fn test_usage() {
        let ability = demand::<testing::RestrictedRule>(statics::all(CREATURE));
        assert_eq!(ability.filter, CREATURE);
        assert_eq!(
            ability.change,
            effects::Change::Demand(rules::RuleId("test/restriction"))
        );
    }
}
