use atomic_delegate_macros::atomic_delegate;

use crate::rules;
use crate::rules::traits::legality_rule::LegalityRule;

pub const CREATURE: rules::ClauseId<rules::AttackCtx> = rules::ClauseId::new("508.1a/creature");
pub const CONTROLS: rules::ClauseId<rules::AttackCtx> = rules::ClauseId::new("508.1a/controls");
pub const UNTAPPED: rules::ClauseId<rules::AttackCtx> = rules::ClauseId::new("508.1a/untapped");
pub const SUMMONING_SICK: rules::ClauseId<rules::AttackCtx> =
    rules::ClauseId::new("508.1a/controlled-since-turn-began");
pub const DEFENDER: rules::ClauseId<rules::AttackCtx> = rules::ClauseId::new("702.3b/defender");

pub struct DeclareAttacker;

impl LegalityRule for DeclareAttacker {
    type Ctx = rules::AttackCtx;
    const ID: rules::RuleId = rules::RuleId("508.1a");

    #[atomic_delegate(DeclareAttacker, LegalityRule)]
    fn clauses() -> Vec<rules::Clause> {}
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::rules;
    use crate::rules::traits::legality_rule::LegalityRule;

    use super::CONTROLS;
    use super::CREATURE;
    use super::DEFENDER;
    use super::DeclareAttacker;
    use super::SUMMONING_SICK;
    use super::UNTAPPED;

    #[test]
    fn test_usage() {
        assert_eq!(DeclareAttacker::ID, rules::RuleId("508.1a"));
        let clauses = DeclareAttacker::clauses();
        assert_eq!(clauses.len(), 5);
        assert_eq!(clauses[0].key, CREATURE.key);
        assert_eq!(
            clauses[0].text,
            "The active player chooses which creatures that they control"
        );
        assert_eq!(clauses[0].role, rules::Role::Subject);
        assert_eq!(clauses[0].requires, rules::has::<cards::Creature>());
        assert_eq!(clauses[1].key, CONTROLS.key);
        assert_eq!(clauses[1].text, "creatures that they control");
        assert_eq!(clauses[1].role, rules::Role::Subject);
        assert_eq!(
            clauses[1].requires,
            rules::points_to::<cards::Owner>(rules::Role::Player)
                & rules::has::<cards::OnBattlefield>()
        );
        assert_eq!(clauses[2].key, UNTAPPED.key);
        assert_eq!(clauses[2].text, "The chosen creatures must be untapped");
        assert_eq!(clauses[2].role, rules::Role::Subject);
        assert_eq!(clauses[2].requires, !rules::has::<cards::Tapped>());
        assert_eq!(clauses[3].key, SUMMONING_SICK.key);
        assert_eq!(clauses[3].role, rules::Role::Subject);
        assert_eq!(clauses[3].requires, !rules::has::<cards::SummoningSick>());
        assert_eq!(clauses[4].key, DEFENDER.key);
        assert_eq!(clauses[4].text, "A creature with defender can’t attack.");
        assert_eq!(clauses[4].role, rules::Role::Subject);
        assert_eq!(clauses[4].requires, !rules::has::<cards::Defender>());
    }

    #[test]
    fn clause_texts_occur_verbatim_in_comprehensive_rules() {
        const RULES_TEXT: &str = include_str!("../../../../docs/MagicCompRules-20250919.txt");
        for clause in rules::DeclareBlocker::clauses().into_iter().chain(DeclareAttacker::clauses())
        {
            assert!(
                RULES_TEXT.contains(clause.text),
                "clause {:?} text not found verbatim in Comprehensive Rules: {:?}",
                clause.key,
                clause.text
            );
        }
    }
}
