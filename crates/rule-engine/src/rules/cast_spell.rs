use atomic_delegate_macros::atomic_delegate;

use crate::rules;
use crate::rules::traits::legality_rule::LegalityRule;

pub const PRIORITY: rules::ClauseId<rules::CastSpellCtx> = rules::ClauseId::new("601.2a/priority");
pub const FROM_HAND: rules::ClauseId<rules::CastSpellCtx> =
    rules::ClauseId::new("601.2a/from-hand");
pub const OWNS: rules::ClauseId<rules::CastSpellCtx> = rules::ClauseId::new("601.2a/owns");
pub const NOT_A_LAND: rules::ClauseId<rules::CastSpellCtx> =
    rules::ClauseId::new("601.2a/not-a-land");
pub const TIMING: rules::ClauseId<rules::CastSpellCtx> = rules::ClauseId::new("601.2a/timing");

pub struct CastSpell;

impl LegalityRule for CastSpell {
    type Ctx = rules::CastSpellCtx;
    const ID: rules::EventId = rules::EventId("601.2a");

    #[atomic_delegate(CastSpell, LegalityRule)]
    fn clauses() -> Vec<rules::Clause> {}
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::cards;
    use crate::rules;
    use crate::rules::traits::legality_rule::LegalityRule;

    use super::CastSpell;
    use super::FROM_HAND;
    use super::NOT_A_LAND;
    use super::OWNS;
    use super::PRIORITY;
    use super::TIMING;

    #[test]
    fn test_usage() {
        assert_eq!(CastSpell::ID, rules::EventId("601.2a"));
        let clauses = CastSpell::clauses();
        assert_eq!(clauses.len(), 5);
        assert_eq!(clauses[0].key, PRIORITY.key);
        assert_eq!(clauses[0].text, "A player who has priority");
        assert_eq!(clauses[0].role, rules::Role::Player);
        assert_eq!(
            clauses[0].condition,
            rules::has_priority(rules::Role::Player)
        );
        assert_eq!(clauses[1].key, FROM_HAND.key);
        assert_eq!(clauses[1].text, "from their hand");
        assert_eq!(clauses[1].role, rules::Role::Player);
        assert_eq!(
            clauses[1].condition,
            rules::in_zone(HAND, rules::Role::Player)
        );
        assert_eq!(clauses[2].key, OWNS.key);
        assert_eq!(clauses[2].text, "their hand");
        assert_eq!(clauses[2].role, rules::Role::Player);
        assert_eq!(
            clauses[2].condition,
            rules::points_to::<cards::Owner>(rules::Role::Player)
        );
        assert_eq!(clauses[3].key, NOT_A_LAND.key);
        assert_eq!(clauses[3].text, "It can’t be cast as a spell.");
        assert_eq!(clauses[3].role, rules::Role::Subject);
        assert_eq!(
            clauses[3].condition,
            !rules::card_type(cards::CardType::Land)
        );
        assert!(clauses[3].absolute);
        assert_eq!(clauses[4].key, TIMING.key);
        assert_eq!(
            clauses[4].text,
            "during a main phase of their turn when the stack is empty"
        );
        assert_eq!(clauses[4].role, rules::Role::Player);
        assert_eq!(
            clauses[4].condition,
            rules::card_type(cards::CardType::Instant)
                | (rules::main_phase()
                    & rules::stack_empty()
                    & rules::active_player(rules::Role::Player))
        );
        assert!(
            CastSpell::clauses()
                .iter()
                .filter(|clause| clause.key != NOT_A_LAND.key)
                .all(|clause| !clause.absolute)
        );
    }

    #[test]
    fn clause_texts_occur_verbatim_in_comprehensive_rules() {
        const RULES_TEXT: &str = include_str!("../../../../docs/MagicCompRules-20250919.txt");
        for clause in CastSpell::clauses() {
            assert!(
                RULES_TEXT.contains(clause.text),
                "clause {:?} text not found verbatim in Comprehensive Rules: {:?}",
                clause.key,
                clause.text
            );
        }
    }
}
