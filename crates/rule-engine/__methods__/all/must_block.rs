use crate::abilities;
use crate::effects;
use crate::rules;
use crate::rules::traits::legality_rule::LegalityRule;
use crate::statics;

pub fn must_block(all: statics::All) -> abilities::Continuous {
    abilities::Continuous {
        filter: *all,
        change: effects::Change::Demand(rules::DeclareBlocker::ID),
    }
}

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::effects;
    use crate::rules;
    use crate::rules::traits::legality_rule::LegalityRule;
    use crate::statics;

    use super::must_block;

    #[test]
    fn test_usage() {
        let ability = must_block(statics::all(CREATURE));
        assert_eq!(ability.filter, CREATURE);
        assert_eq!(
            ability.change,
            effects::Change::Demand(rules::DeclareBlocker::ID)
        );
        assert_eq!(rules::DeclareBlocker::ID, rules::RuleId("509.1a"));
    }
}
