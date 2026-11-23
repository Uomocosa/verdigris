use crate::cards;
use crate::rules;

pub fn legality_rule_clauses() -> Vec<rules::Clause> {
    vec![
        rules::declare_blocker::CREATURE.clause(
            "The defending player chooses which creatures they control",
            rules::Role::Subject,
            rules::has::<cards::Creature>(),
        ),
        rules::declare_blocker::CONTROLS.clause(
            "creatures they control",
            rules::Role::Subject,
            rules::points_to::<cards::Owner>(rules::Role::Player)
                & rules::has::<cards::OnBattlefield>(),
        ),
        rules::declare_blocker::UNTAPPED.clause(
            "The chosen creatures must be untapped",
            rules::Role::Subject,
            !rules::has::<cards::Tapped>(),
        ),
        rules::declare_blocker::FLYING.clause(
            "can’t be blocked except by creatures with flying and/or reach",
            rules::Role::Other,
            !rules::other(rules::has::<cards::Flying>())
                | rules::has::<cards::Flying>()
                | rules::has::<cards::Reach>(),
        ),
    ]
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::rules;

    use super::legality_rule_clauses;

    #[test]
    fn test_usage() {
        let clauses = legality_rule_clauses();
        assert_eq!(clauses.len(), 4);
        assert_eq!(clauses[0].key, rules::ClauseKey("509.1a/creature"));
        assert_eq!(clauses[0].requires, rules::has::<cards::Creature>());
        assert_eq!(clauses[1].key, rules::ClauseKey("509.1a/controls"));
        assert_eq!(
            clauses[1].requires,
            rules::points_to::<cards::Owner>(rules::Role::Player)
                & rules::has::<cards::OnBattlefield>()
        );
        assert_eq!(clauses[2].key, rules::ClauseKey("509.1a/untapped"));
        assert_eq!(clauses[2].requires, !rules::has::<cards::Tapped>());
        assert_eq!(clauses[3].key, rules::ClauseKey("702.9b/flying"));
        assert_eq!(
            clauses[3].requires,
            !rules::other(rules::has::<cards::Flying>())
                | rules::has::<cards::Flying>()
                | rules::has::<cards::Reach>()
        );
    }
}
