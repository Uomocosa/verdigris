use std::collections::HashSet;

use crate::rules;
use crate::rules::traits::event::Event;
use crate::rules::traits::value_rule::ValueRule;
#[cfg(any(test, feature = "testing"))]
use crate::testing;

pub fn engine() -> rules::RuleRegistry {
    let mut events = HashSet::<rules::EventId>::new();
    let mut clauses = HashSet::<rules::ClauseKey>::new();
    let mut register = |id: rules::EventId, list: Vec<rules::Clause>| {
        events.insert(id);
        for clause in list {
            clauses.insert(clause.key);
        }
    };
    register(rules::CastSpell::ID, rules::CastSpell::clauses());
    register(rules::Counter::ID, rules::Counter::clauses());
    register(rules::CreateTokens::ID, rules::CreateTokens::clauses());
    register(rules::DealDamage::ID, rules::DealDamage::clauses());
    register(
        rules::DeclareAttacker::ID,
        rules::DeclareAttacker::clauses(),
    );
    register(rules::DeclareBlocker::ID, rules::DeclareBlocker::clauses());
    register(rules::Destroy::ID, rules::Destroy::clauses());
    register(rules::Discard::ID, rules::Discard::clauses());
    register(rules::Draw::ID, rules::Draw::clauses());
    register(
        rules::DrawFromEmptyLibrary::ID,
        rules::DrawFromEmptyLibrary::clauses(),
    );
    register(
        rules::EnterBattlefield::ID,
        rules::EnterBattlefield::clauses(),
    );
    register(rules::GainLife::ID, rules::GainLife::clauses());
    register(rules::LoseGame::ID, rules::LoseGame::clauses());
    register(rules::LoseLife::ID, rules::LoseLife::clauses());
    register(rules::Mill::ID, rules::Mill::clauses());
    register(rules::PlayLand::ID, rules::PlayLand::clauses());
    register(rules::TapAttacker::ID, rules::TapAttacker::clauses());
    register(rules::Untap::ID, rules::Untap::clauses());
    register(rules::WinGame::ID, rules::WinGame::clauses());
    #[cfg(any(test, feature = "testing"))]
    {
        register(testing::TestRule::ID, testing::TestRule::clauses());
        register(testing::AbsoluteRule::ID, testing::AbsoluteRule::clauses());
        register(
            testing::RestrictedRule::ID,
            testing::RestrictedRule::clauses(),
        );
        register(testing::TestStep::ID, testing::TestStep::clauses());
    }
    let values = HashSet::from([rules::LandPlays::ID, rules::MaximumHandSize::ID]);
    rules::RuleRegistry {
        events,
        clauses,
        values,
    }
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::value_rule::ValueRule;

    use super::engine;

    #[test]
    fn test_usage() {
        let registry = engine();
        assert!(registry.events.contains(&rules::Draw::ID));
        assert!(registry.clauses.contains(&rules::declare_attacker::SUMMONING_SICK.key));
        assert!(registry.values.contains(&rules::LandPlays::ID));
        assert_eq!(rules::Draw::ID, rules::EventId("121.1"));
    }
}
