use crate::cards;
use crate::rules;

pub fn event_clauses() -> Vec<rules::Clause> {
    vec![
        rules::declare_attacker::CREATURE.clause(
            "The active player chooses which creatures that they control",
            rules::Role::Subject,
            rules::has::<cards::Creature>(),
        ),
        rules::declare_attacker::CONTROLS.clause(
            "creatures that they control",
            rules::Role::Subject,
            rules::points_to::<cards::Owner>(rules::Role::Player)
                & rules::has::<cards::OnBattlefield>(),
        ),
        rules::declare_attacker::UNTAPPED.clause(
            "The chosen creatures must be untapped",
            rules::Role::Subject,
            !rules::has::<cards::Tapped>(),
        ),
        rules::declare_attacker::SUMMONING_SICK.clause(
            "have been controlled by the active player continuously since the turn began",
            rules::Role::Subject,
            !rules::has::<cards::SummoningSick>(),
        ),
        rules::declare_attacker::DEFENDER.clause(
            "A creature with defender can’t attack.",
            rules::Role::Subject,
            !rules::has::<cards::Defender>(),
        ),
    ]
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::rules;

    use super::event_clauses;

    #[test]
    fn test_usage() {
        let clauses = event_clauses();
        assert_eq!(clauses.len(), 5);
        assert_eq!(clauses[0].key, rules::ClauseKey("508.1a/creature"));
        assert_eq!(clauses[0].condition, rules::has::<cards::Creature>());
        assert_eq!(clauses[1].key, rules::ClauseKey("508.1a/controls"));
        assert_eq!(
            clauses[1].condition,
            rules::points_to::<cards::Owner>(rules::Role::Player)
                & rules::has::<cards::OnBattlefield>()
        );
        assert_eq!(clauses[2].key, rules::ClauseKey("508.1a/untapped"));
        assert_eq!(clauses[2].condition, !rules::has::<cards::Tapped>());
        assert_eq!(
            clauses[3].key,
            rules::ClauseKey("508.1a/controlled-since-turn-began")
        );
        assert_eq!(clauses[3].condition, !rules::has::<cards::SummoningSick>());
        assert_eq!(clauses[4].key, rules::ClauseKey("702.3b/defender"));
        assert_eq!(clauses[4].condition, !rules::has::<cards::Defender>());
    }
}
