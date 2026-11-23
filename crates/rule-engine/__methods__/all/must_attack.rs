use crate::abilities;
use crate::effects;
use crate::rules;
use crate::rules::traits::legality_rule::LegalityRule;
use crate::statics;

pub fn must_attack(all: statics::All) -> abilities::Continuous {
    abilities::Continuous {
        filter: *all,
        change: effects::Change::Demand(rules::DeclareAttacker::ID),
    }
}

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::effects;
    use crate::rules;
    use crate::rules::traits::legality_rule::LegalityRule;
    use crate::statics;

    use super::must_attack;

    #[test]
    fn test_usage() {
        let ability = must_attack(statics::all(CREATURE));
        assert_eq!(ability.filter, CREATURE);
        assert_eq!(
            ability.change,
            effects::Change::Demand(rules::DeclareAttacker::ID)
        );
        assert_eq!(rules::DeclareAttacker::ID, rules::RuleId("508.1a"));
    }
}
